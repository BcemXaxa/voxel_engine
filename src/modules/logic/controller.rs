use std::{
    sync::{mpsc::Receiver, Arc},
    time::{Duration, Instant},
};

use winit::{event::WindowEvent, window::Window};

use crate::modules::renderer::{queue::QueueType, Renderer};

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

        let mut last = Instant::now() - frame_duration;
        'main: loop {
            let mut swapchain_recreate = false;
            let mut redraw_requested = false;

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
                        swapchain_recreate = true;
                    }
                    RedrawRequested => {
                        // TODO
                        redraw_requested = true;
                    }
                    _ => (),
                }
            }

            if swapchain_recreate {
                self.renderer
                    .recreate_swapchain(self.window.inner_size().into());
            }
            if redraw_requested || (Instant::now() - last > frame_duration) {
                self.renderer.execute_then_present(command_buffer, QueueType::GraphicsPresent);
                last = Instant::now()
            }
        }
    }

    fn draw_frame(&self) {}
}
