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

use crate::{messenger::{window_interface::InterfaceMessenger, window_renderer::RendererMessenger}, renderer::RendererIncomingEvent};

#[derive(Debug)]
pub enum ApplicationEvent {
    CloseFix,
}

pub struct Application {
    window: Option<Arc<Window>>,

    render_msg: RendererMessenger,
    interface_msg: InterfaceMessenger,

    event_loop: Option<EventLoop<ApplicationEvent>>,
    proxy: EventLoopProxy<ApplicationEvent>,
}

impl Application {
    pub fn new(render_msg: RendererMessenger, interface_msg: InterfaceMessenger) -> Self {
        let event_loop = EventLoop::with_user_event().build().unwrap();
        let proxy = event_loop.create_proxy();
        Self {
            render_msg,
            interface_msg,
            proxy,
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
        
        self.render_msg.init(event_loop, window.clone());
        self.interface_msg.init(event_loop, window.clone());

        self.window = Some(window);
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
            _ => {
                self.render_msg
                    .window_event(event_loop, window, event.clone());
                self.interface_msg
                    .window_event(event_loop, window, event);
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

pub trait WindowEventHandler {
    fn init(&mut self, event_loop: &ActiveEventLoop, window: Arc<Window>) {}
    fn window_event(&mut self, event_loop: &ActiveEventLoop, window: &Window, event: WindowEvent);
}

impl WindowEventHandler for RendererMessenger {
    fn init(&mut self, event_loop: &ActiveEventLoop, window: Arc<Window>) {
        self.initial_sender.send((window, Surface::required_extensions(&event_loop))).unwrap();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window: &Window, event: WindowEvent) {
        match event {
            Resized(physical_size) => {
                let extent = physical_size.into();
                self.run_sender.send(RendererIncomingEvent::ExtentChange(extent));
            },
            _ => (),
        }
    }
}

impl WindowEventHandler for InterfaceMessenger {
    fn window_event(&mut self, event_loop: &ActiveEventLoop, window: &Window, event: WindowEvent) {
        
    }
}