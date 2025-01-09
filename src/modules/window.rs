use std::{
    collections::HashMap,
    sync::{mpsc::Sender, Arc},
};

use winit::{
    application::ApplicationHandler,
    event::{
        DeviceEvent, DeviceId,
        WindowEvent::{self, *},
    },
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    window::{Window, WindowAttributes, WindowId},
};

pub enum CustomEvent {
    Exit,
    CreateWindow(
        WindowAttributes,
        Sender<Arc<Window>>,
        Sender<WindowEvent>,
        Sender<DeviceEvent>,
    ),
}

pub struct WindowManagerBuilder {
    event_loop: EventLoop<CustomEvent>,
}
impl Default for WindowManagerBuilder {
    fn default() -> Self {
        let event_loop = EventLoop::with_user_event().build().unwrap();

        Self { event_loop }
    }
}
impl WindowManagerBuilder {
    pub fn build_and_run(self) {
        let mut window_manager = WindowManager {
            windows: HashMap::new(),
            local_proxy: self.event_loop.create_proxy(),
        };
        self.event_loop.run_app(&mut window_manager).unwrap();
    }

    pub fn event_loop_proxy(&self) -> EventLoopProxy<CustomEvent> {
        self.event_loop.create_proxy()
    }

    pub fn event_loop(&self) -> &EventLoop<CustomEvent> {
        &self.event_loop
    }
}

pub struct WindowManager {
    windows: HashMap<WindowId, (Arc<Window>, Sender<WindowEvent>, Sender<DeviceEvent>)>,
    local_proxy: EventLoopProxy<CustomEvent>,
}

impl ApplicationHandler<CustomEvent> for WindowManager {
    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            CloseRequested => {
                if let Some((window, _, _)) = self.windows.get(&window_id) {
                    if let Some(false) = window.is_visible() {
                        window.set_visible(true);
                    }
                }
            }
            Destroyed => {
                self.windows.remove(&window_id);
                if self.windows.is_empty() {
                    self.local_proxy.send_event(CustomEvent::Exit);
                }
            }
            _ => (),
        }

        if let Some((_, sender, _)) = self.windows.get(&window_id) {
            // Send event
            // Destroy window if no receiver
            if sender.send(event).is_err() {
                self.windows.remove(&window_id);
            }
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {
        for (_, _, sender) in self.windows.values() {
            // Send event
            // Print message if no receiver
            if sender.send(event.clone()).is_err() {
                // TODO(handle_error)
                println!("No receiver")
            }
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: CustomEvent) {
        use CustomEvent::*;
        match event {
            Exit => event_loop.exit(),
            CreateWindow(
                window_attributes,
                window_sender,
                window_event_sender,
                device_event_sender,
            ) => {
                let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
                window_sender.send(window.clone());
                self.windows.insert(
                    window.id(),
                    (window, window_event_sender, device_event_sender),
                );
            }
        }
    }

    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        // no need
    }
}
