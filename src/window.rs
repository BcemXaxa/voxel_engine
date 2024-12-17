use std::sync::{mpsc::Sender, Arc};

use vulkano::{instance::InstanceExtensions, swapchain::Surface};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent::{self, *},
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    window::{Window, WindowAttributes, WindowId},
};
use ApplicationEvent::*;

use crate::renderer::Renderer;

#[derive(Debug)]
pub enum ApplicationEvent {
    CloseFix,
}

pub struct Application {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,

    event_loop: Option<EventLoop<ApplicationEvent>>, // TODO: probably unneccessary
    proxy: EventLoopProxy<ApplicationEvent>,
}

impl Application {
    pub fn new() -> Self {
        let event_loop = EventLoop::with_user_event().build().unwrap();
        let proxy = event_loop.create_proxy();
        Self {
            proxy,
            renderer: None,
            event_loop: Some(event_loop),
            window: None,
        }
    }
    pub fn run(mut self) {
        let event_loop = self.event_loop.take().unwrap();
        event_loop
            .run_app(&mut self)
            .expect("Window application result");
    }
}

impl ApplicationHandler<ApplicationEvent> for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_title("title")
            .with_inner_size(LogicalSize::new(800, 600));

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        self.window = Some(window.clone());
        self.renderer = Some(Renderer::new(window, Surface::required_extensions(&event_loop)));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let window = self.window.as_ref().unwrap();
        match event {
            CloseRequested => {
                if let Some(false) = window.is_visible() {
                    window.set_visible(true);
                }
                self.proxy.send_event(CloseFix).expect("Proxy send failed");
            }
            RedrawRequested => {
                println!("redraw");
                self.renderer.as_mut().unwrap().recreate(None);
                self.renderer.as_mut().unwrap().draw();
            }
            _ => {
            }
        }
    }
    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: ApplicationEvent) {
        match event {
            CloseFix => event_loop.exit(),
            _ => (),
        }
    }
}
