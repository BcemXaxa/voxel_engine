#![allow(unused)]

mod modules;

use std::{sync::mpsc, thread, time::Duration};

use modules::logic::controller::Controller;
use modules::renderer::{self, Renderer};
use modules::window::{CustomEvent, WindowManager, WindowManagerBuilder};
use vulkano::swapchain::Surface;
use winit::dpi::LogicalSize;
use winit::window::WindowAttributes;

fn main() {
    let window_manager_builder = WindowManagerBuilder::default();
    let required_extensions = Surface::required_extensions(window_manager_builder.event_loop());
    let (window_send, window_recv) = mpsc::channel();
    let (event_send, event_recv) = mpsc::channel();
    let proxy = window_manager_builder.event_loop_proxy();
    thread::spawn(move || {
        let window_attributes = WindowAttributes::default()
            .with_title("title")
            .with_inner_size(LogicalSize::new(800, 600));
        proxy.send_event(CustomEvent::CreateWindow(
            window_attributes,
            window_send,
            event_send,
        ));
        let window = window_recv.recv().unwrap();
        let renderer = Renderer::new(window.clone(), required_extensions);
        let mut controller = Controller::new(window, event_recv, renderer);
        controller.main_loop();
        proxy.send_event(CustomEvent::Exit);
    });
    window_manager_builder.build_and_run();
}
