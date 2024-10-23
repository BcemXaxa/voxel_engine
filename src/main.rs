#![allow(unused)]
mod for_multi;
//mod renderer;
mod window;

//use renderer::Renderer;
use vulkano::swapchain::Surface;
use window::WindowManager;
use winit::{event_loop::{self, EventLoop}, raw_window_handle::{HasDisplayHandle}};

fn main() {
    let mut window_manager = WindowManager::default();
    window_manager.run();
}
