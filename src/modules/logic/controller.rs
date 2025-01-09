use std::{
    cell::Cell,
    rc::Rc,
    sync::{mpsc::Receiver, Arc},
};

use winit::{
    event::WindowEvent,
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use crate::modules::{math::vec::VecAdd, renderer::Renderer, utility::framerate::Framerate};

use super::{key_input::KeyInputHelper, render_controller::RenderController, scene::Scene};

pub struct Controller {
    window: Arc<Window>,
    events: Receiver<WindowEvent>,

    scene: Rc<Scene>,
    render_controller: RenderController,
}

impl Controller {
    pub fn new(window: Arc<Window>, events: Receiver<WindowEvent>, renderer: Renderer) -> Self {
        let scene = Rc::new(Scene::default());
        Self {
            window,
            events,
            scene: scene.clone(),
            render_controller: RenderController::new(renderer, scene),
        }
    }

    pub fn main_loop(&mut self) {
        let mut input = KeyInputHelper::default();

        let mut framerate = Framerate::new(None);
        let mut fixed = Framerate::new(Some(60.0));
        let mut console_stat = Framerate::new(Some(1.0));

        'main: loop {
            let mut redraw_request = false;
            let mut resized = Option::None;

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

            if fixed.should_render() {
                fixed.refresh();
                
                if input.is_pressed(KeyCode::KeyW) {
                    let pos = self.scene.camera.borrow().pos;
                    self.scene.camera.borrow_mut().pos = pos.add([0.0, 0.5, 0.0]);
                }
                if input.is_pressed(KeyCode::KeyS) {
                    let pos = self.scene.camera.borrow().pos;
                    self.scene.camera.borrow_mut().pos = pos.add([0.0, -0.5, 0.0]);
                }
                if input.is_pressed(KeyCode::KeyA) {
                    let pos = self.scene.camera.borrow().pos;
                    self.scene.camera.borrow_mut().pos = pos.add([-0.5, 0.0, 0.0]);
                }
                if input.is_pressed(KeyCode::KeyD) {
                    let pos = self.scene.camera.borrow().pos;
                    self.scene.camera.borrow_mut().pos = pos.add([0.5, 0.0, 0.0]);
                }
                if input.is_pressed(KeyCode::Space) {
                    let pos = self.scene.camera.borrow().pos;
                    self.scene.camera.borrow_mut().pos = pos.add([0.0, 0.0, 0.5]);
                }
                if input.is_pressed(KeyCode::ControlLeft) {
                    let pos = self.scene.camera.borrow().pos;
                    self.scene.camera.borrow_mut().pos = pos.add([0.0, 0.0, -0.5]);
                }
                if input.is_pressed(KeyCode::Minus) {
                    self.render_controller.fov_minus();
                }
                if input.is_pressed(KeyCode::Equal) {
                    self.render_controller.fov_plus();
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
