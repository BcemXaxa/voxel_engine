use std::{cmp::Reverse, collections::BinaryHeap, time::{Duration, Instant}};

pub struct Framerate {
    target: Option<Duration>,
    last: Instant,

    stamps: BinaryHeap<Reverse<Instant>>,
}

impl Framerate {
    pub fn new(target_fps: Option<f32>) -> Self {
        Self {
            target: target_fps.map(|fps| Duration::from_secs_f32(1.0 / fps)),
            last: Instant::now(),

            stamps: BinaryHeap::with_capacity(66),
        }
    }

    pub fn refresh(&mut self) {
        self.stamp();
        if let Some(time) = self.target {
            let this = self.last + time;
            if this.elapsed() < time {
                self.last = this;
                return
            }
        }
        self.last = Instant::now();
    }

    pub fn should_render(&self) -> bool {
        self.target
            .as_ref()
            .map_or(true, |target| *target <= self.since_last())
    }

    fn since_last(&self) -> Duration {
        self.last.elapsed()
    }

    fn stamp(&mut self) {
        self.stamps.push(Reverse(Instant::now()));
        while self
            .stamps
            .peek()
            .map_or(false, |time| time.0.elapsed() > Duration::from_secs(1))
        {
            self.stamps.pop();
        }
    }

    pub fn fps(&self) -> u32 {
        self.stamps.len() as u32
    }

    pub fn frame_time(&self) -> f32 {
        1.0 / self.fps() as f32
    }
}
