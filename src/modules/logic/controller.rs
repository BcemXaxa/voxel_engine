use std::{
    cell::Cell, rc::Rc, sync::{mpsc::Receiver, Arc}
};

use winit::{event::WindowEvent, keyboard::{KeyCode, PhysicalKey}, window::Window};

use crate::modules::{math::vec::VecAdd, renderer::Renderer, utility::framerate::Framerate};

use super::{render_controller::RenderController, scene::Scene};

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
        let mut framerate = Framerate::new(Some(60.0));

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
                        if event.physical_key == KeyCode::KeyW && event.state.is_pressed() {
                            let pos = self.scene.camera.borrow().pos;
                            self.scene.camera.borrow_mut().pos = pos.add([0.0, 1.0, 0.0]);
                        }
                        if event.physical_key == KeyCode::KeyS && event.state.is_pressed() {
                            let pos = self.scene.camera.borrow().pos;
                            self.scene.camera.borrow_mut().pos = pos.add([0.0, -1.0, 0.0]);
                        }
                        if event.physical_key == KeyCode::KeyA && event.state.is_pressed() {
                            let pos = self.scene.camera.borrow().pos;
                            self.scene.camera.borrow_mut().pos = pos.add([-1.0, 0.0, 0.0]);
                        }
                        if event.physical_key == KeyCode::KeyD && event.state.is_pressed() {
                            let pos = self.scene.camera.borrow().pos;
                            self.scene.camera.borrow_mut().pos = pos.add([1.0, 0.0, 0.0]);
                        }
                        if event.physical_key == KeyCode::Space && event.state.is_pressed() {
                            let pos = self.scene.camera.borrow().pos;
                            self.scene.camera.borrow_mut().pos = pos.add([0.0, 0.0, 1.0]);
                        }
                        if event.physical_key == KeyCode::ShiftLeft && event.state.is_pressed() {
                            let pos = self.scene.camera.borrow().pos;
                            self.scene.camera.borrow_mut().pos = pos.add([0.0, 0.0, -1.0]);
                        }
                        if event.physical_key == KeyCode::Minus && event.state.is_pressed() {
                            self.render_controller.fov_minus();
                        }
                        if event.physical_key == KeyCode::Equal && event.state.is_pressed() {
                            self.render_controller.fov_plus();
                        }
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
                        resized = Some(physical_size);
                        // TODO
                    }
                    RedrawRequested => {
                        redraw_request = true;
                    }
                    _ => (),
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
        }
    }
}
