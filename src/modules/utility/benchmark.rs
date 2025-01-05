use std::{
    iter::Peekable,
    time::{Duration, Instant},
    vec::IntoIter,
};

struct Timestamp {
    stat: Marker,
    time: Instant,
}

enum Marker {
    Begin(String),
    End,
}

impl Marker {
    fn is_begin(&self) -> bool {
        match self {
            Marker::Begin(_) => true,
            _ => false,
        }
    }
    fn is_end(&self) -> bool {
        match self {
            Marker::End => true,
            _ => false,
        }
    }
}

pub struct ActiveTimeline {
    stamps: Vec<Timestamp>,
}

impl ActiveTimeline {
    const DEFAULT_CAPACITY: usize = 20;
    fn new() -> Self {
        Self::with_capacity(Self::DEFAULT_CAPACITY)
    }
    fn with_capacity(capacity: usize) -> Self {
        Self {
            stamps: Vec::with_capacity(capacity),
        }
    }

    pub fn begin(&mut self, name: &str) {
        self.stamps.push(Timestamp {
            stat: Marker::Begin(name.to_string()),
            time: Instant::now(),
        });
    }

    pub fn end(&mut self) {
        self.stamps.push(Timestamp {
            stat: Marker::End,
            time: Instant::now(),
        });
    }

    pub fn complete(self) -> Result<CompleteTimeline, &'static str> {
        let begin = self.stamps.first().expect("Empty Timeline").time;
        let end = self.stamps.last().expect("Empty Timeline").time;
        let mut iter = self.stamps.into_iter().peekable();
        if let Ok(sub_measures) = Self::measures(&mut iter) {
            if iter.next().is_some() {
                return Err("Invalid Timeline");
            }
            let mut measure = Measure::new("Total".to_string(), begin, end);
            measure.sub = sub_measures;
            Ok(CompleteTimeline { total: measure })
        } else {
            Err("Invalid Timeline")
        }
    }

    fn measures(iter: &mut Peekable<IntoIter<Timestamp>>) -> Result<Vec<Measure>, ()> {
        let mut measures = Vec::new();

        while let Some(Timestamp {
            stat: Marker::Begin(name),
            time: begin,
        }) = iter.next_if(|timestamp| timestamp.stat.is_begin())
        {
            if let (
                Ok(sub_measures),
                Some(Timestamp {
                    stat: Marker::End,
                    time: end,
                }),
            ) = (Self::measures(iter), iter.next())
            {
                let mut measure = Measure::new(name, begin, end);
                measure.sub = sub_measures;
                measures.push(measure);
            } else {
                return Err(());
            }
        }

        Ok(measures)
    }
}

#[derive(Debug)]
struct Measure {
    name: String,
    duration: Duration,
    sub: Vec<Measure>,
}
impl Measure {
    fn new(name: String, begin: Instant, end: Instant) -> Self {
        Self {
            name,
            duration: end.duration_since(begin),
            sub: Vec::new(),
        }
    }
}
pub struct CompleteTimeline {
    total: Measure,
}

impl CompleteTimeline {
    fn print(&self) {
        Self::print_measure(&self.total, 0, self.total.duration);
    }

    fn print_measure(measure: &Measure, depth: usize, parent_duration: Duration) {
        println!(
            "{}{:.2}%: {:?} - {}",
            "|      ".repeat(depth),
            measure.duration.div_duration_f32(parent_duration) * 100.0,
            measure.duration,
            measure.name
        );
        for sub_measure in &measure.sub {
            Self::print_measure(sub_measure, depth + 1, measure.duration);
        }
    }
}

#[cfg(test)]
mod benchmark_tests {
    use super::ActiveTimeline;

    #[test]
    fn print_test() {
        let mut timeline = ActiveTimeline::new();
        timeline.begin("Event Handling");
        {
            timeline.begin("Window");
            {
                timeline.begin("Movement");
                timeline.end();
                timeline.begin("Cursor");
                timeline.end();
            }
            timeline.end();
            timeline.begin("Custom");
            {
                timeline.begin("Physics");
                let mut k = 0;
                for _ in 0..1_000_000 {
                    k += 1;
                }
                timeline.end();
            }
            timeline.end();
        }
        timeline.end();
        timeline.begin("Rendering");
        {
            timeline.begin("Swapchain");
            timeline.end();
            timeline.begin("Render Pass");
            timeline.end();
            timeline.begin("Pipeline");
            timeline.end();
            timeline.begin("Commands");
            timeline.end();
        }
        timeline.end();

        timeline.complete().unwrap().print();
    }
}
