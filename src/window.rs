use winit::{
    self,
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent::{self, *},
    event_loop::{self, ActiveEventLoop, EventLoop},
    raw_window_handle::HasDisplayHandle,
    window::{Window, WindowAttributes, WindowId},
};

pub struct WindowManager {
    window: Option<Window>,
}

impl WindowManager {
    pub fn run(&mut self){
        let event_loop = EventLoop::new().unwrap();
        // FIXME: blocks current thread
        event_loop.run_app(self);
    }
}

impl Default for WindowManager {
    fn default() -> Self {
        Self {
            window: None,
        }
    }
}

impl ApplicationHandler for WindowManager {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(_) = self.window {
            println!("resumed");
        } else {
            let window_attributes = WindowAttributes::default()
                .with_title("title")
                .with_inner_size(LogicalSize::new(800, 600));

            self.window = Some(event_loop.create_window(window_attributes).unwrap());
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let mut window = self.window.as_ref().unwrap();

        match event {
            CloseRequested => event_loop.exit(),
            RedrawRequested => {
                window.request_redraw();
            }
            _ => (),
        }
    }
}
