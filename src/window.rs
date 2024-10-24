use std::sync::{mpsc::Sender, Arc};

use vulkano::{instance::InstanceExtensions, swapchain::Surface};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent::{self, *},
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    window::{Window, WindowAttributes, WindowId},
};
use ApplicationEvents::*;

#[derive(Debug)]
pub enum ApplicationEvents {
    CloseFix,
    CommonEvent(WindowEvent),
    RenderEvent(WindowEvent),
    InterfaceEvent(WindowEvent),
}

pub struct Application {
    window: Option<Arc<Window>>,

    render_manager: RenderManager,
    interface_manager: InterfaceManager,

    event_loop: Option<EventLoop<ApplicationEvents>>,
    proxy: EventLoopProxy<ApplicationEvents>,
}

impl Application {
    pub fn new(render_manager: RenderManager, interface_manager: InterfaceManager) -> Self {
        let event_loop = EventLoop::with_user_event().build().unwrap();
        let proxy = event_loop.create_proxy();
        Self {
            render_manager,
            interface_manager,
            proxy,
            event_loop: Some(event_loop),
            window: None,
        }
    }
    pub fn run(mut self) {
        let event_loop = self.event_loop.unwrap();
        self.event_loop = None;
        event_loop
            .run_app(&mut self)
            .expect("Window application result");
    }
}

impl ApplicationHandler<ApplicationEvents> for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_title("title")
            .with_inner_size(LogicalSize::new(800, 600));

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        self.render_manager.init(event_loop, window.clone());
        self.interface_manager.init(event_loop, window.clone());

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
                self.render_manager
                    .window_event(event_loop, window, event.clone());
                self.interface_manager
                    .window_event(event_loop, window, event);
            }
        }
    }
    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: ApplicationEvents) {
        match event {
            CloseFix => event_loop.exit(),
            _ => (),
        }
    }
}

pub trait WindowManager {
    fn init(&mut self, event_loop: &ActiveEventLoop, window: Arc<Window>) {}
    fn window_event(&mut self, event_loop: &ActiveEventLoop, window: &Window, event: WindowEvent) {}
    fn application_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window: &Window,
        event: WindowEvent,
    ) {
    }
}

pub struct RenderManager {
    pub required_extensions_sender: Sender<InstanceExtensions>,
    pub window_sender: Sender<Arc<Window>>,
}
impl WindowManager for RenderManager {
    fn init(&mut self, event_loop: &ActiveEventLoop, window: Arc<Window>) {
        self.required_extensions_sender.send(Surface::required_extensions(&event_loop)).unwrap();
        self.window_sender.send(window).unwrap();
    }
}

pub struct InterfaceManager {}
impl WindowManager for InterfaceManager {}
