use std::{
    cell::Cell,
    rc::Rc,
    sync::{mpsc::Receiver, Arc},
};

use winit::{
    event::{DeviceEvent, Event, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use crate::modules::{math::vec::VecAdd, renderer::Renderer, utility::framerate::Framerate};

use super::{key_input::KeyInputHelper, render_controller::RenderController, scene::Scene};

pub struct Controller {
    window: Arc<Window>,
    window_events: Receiver<WindowEvent>,
    device_events: Receiver<DeviceEvent>,

    scene: Rc<Scene>,
    render_controller: RenderController,
}

impl Controller {
    pub fn new(
        window: Arc<Window>,
        renderer: Renderer,
        window_events: Receiver<WindowEvent>,
        device_events: Receiver<DeviceEvent>,
    ) -> Self {
        let scene = Rc::new(Scene::default());
        Self {
            window,
            window_events,
            device_events,
            scene: scene.clone(),
            render_controller: RenderController::new(renderer, scene),
        }
    }

    pub fn main_loop(&mut self) {
        let mut input = KeyInputHelper::default();

        let mut framerate = Framerate::new(Some(60.0));
        let mut fixed = Framerate::new(Some(60.0));
        let mut console_stat = Framerate::new(Some(1.0));

        'main: loop {
            let mut redraw_request = false;
            let mut resized = Option::None;

            let window_events = self.window_events.try_iter();
            for event in window_events {
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
                        input.input(event);
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
                        resized = Some(physical_size);
                        // TODO
                    }
                    RedrawRequested => {
                        redraw_request = true;
                    }
                    _ => (),
                }
            }

            let device_events = self.device_events.try_iter();
            for event in device_events {
                match event {
                    DeviceEvent::MouseMotion { delta } => {
                        self.scene
                            .camera
                            .borrow_mut()
                            .local_rotate([delta.0 as f32, delta.1 as f32]);
                    }
                    _ => (),
                }
            }

            if fixed.should_render() {
                fixed.refresh();

                if input.is_pressed(KeyCode::KeyQ) {
                    self.scene.camera.borrow_mut().local_roll(1.5);
                }
                if input.is_pressed(KeyCode::KeyE) {
                    self.scene.camera.borrow_mut().local_roll(-1.5);
                }

                if input.is_pressed(KeyCode::KeyW) {
                    self.scene.camera.borrow_mut().local_move([0.0, 0.5, 0.0]);
                }
                if input.is_pressed(KeyCode::KeyS) {
                    self.scene.camera.borrow_mut().local_move([0.0, -0.5, 0.0]);
                }
                if input.is_pressed(KeyCode::KeyA) {
                    self.scene.camera.borrow_mut().local_move([-0.5, 0.0, 0.0]);
                }
                if input.is_pressed(KeyCode::KeyD) {
                    self.scene.camera.borrow_mut().local_move([0.5, 0.0, 0.0]);
                }
                if input.is_pressed(KeyCode::Space) {
                    self.scene.camera.borrow_mut().local_move([0.0, 0.0, 0.5]);
                }
                if input.is_pressed(KeyCode::ControlLeft) {
                    self.scene.camera.borrow_mut().local_move([0.0, 0.0, -0.5]);
                }
                if input.is_pressed(KeyCode::Minus) {
                    self.render_controller.fov_minus();
                }
                if input.is_pressed(KeyCode::Equal) {
                    self.render_controller.fov_plus();
                }

                if input.is_pressed(KeyCode::Escape) {
                    break 'main;
                }
            }

            if let Some(physical_size) = resized {
                // TODO
                self.render_controller.extent_changed(physical_size.into());
            }
            if redraw_request || framerate.should_render() {
                framerate.refresh();
                self.render_controller.draw_frame();
            }
            if console_stat.should_render() {
                console_stat.refresh();
                println!(
                    "FPS: {}; Frame: {:?}",
                    framerate.fps(),
                    framerate.frame_time()
                );
            }
        }
    }
}
