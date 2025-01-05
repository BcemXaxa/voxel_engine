use std::{
    sync::{mpsc::Receiver, Arc},
    time::{Duration, Instant},
};

use winit::{event::WindowEvent, window::Window};

use crate::modules::renderer::Renderer;

use super::scene::Scene;

pub struct Controller {
    window: Arc<Window>,
    events: Receiver<WindowEvent>,
    renderer: Renderer,
    scene: Scene,
}

impl Controller {
    pub fn new(window: Arc<Window>, events: Receiver<WindowEvent>, renderer: Renderer) -> Self {
        Self {
            window,
            events,
            renderer,
            scene: Scene::default(),
        }
    }

    pub fn main_loop(&mut self) {
        let frame_duration = Duration::from_secs_f32(1.0 / 60.0);
        let mut last_frame = Instant::now();

        'main: loop {
            let events = self.events.try_iter();
            for event in events {
                use WindowEvent::*;
                match event {
                    CloseRequested => {
                        // TODO
                        break 'main;
                    }
                    Destroyed => {
                        // TODO
                    }
                    Focused(_) => {
                        // TODO
                    }
                    KeyboardInput {
                        device_id,
                        event,
                        is_synthetic,
                    } => {
                        // TODO
                    }
                    CursorMoved {
                        device_id,
                        position,
                    } => {
                        // TODO
                    }
                    CursorEntered { device_id } => {
                        // TODO
                    }
                    CursorLeft { device_id } => {
                        // TODO
                    }
                    MouseWheel {
                        device_id,
                        delta,
                        phase,
                    } => {
                        // TODO
                    }
                    MouseInput {
                        device_id,
                        state,
                        button,
                    } => {
                        // TODO
                    }
                    Resized(physical_size) => {
                        // TODO
                    }
                    RedrawRequested => {
                        last_frame = Instant::now();
                        // TODO: render frame
                    }
                    _ => (),
                }
            }

            let now = Instant::now();
            let since_last = now.duration_since(last_frame);
            if since_last >= frame_duration {
                last_frame = now;
                // TODO: render frame
            }
        }
    }
}
