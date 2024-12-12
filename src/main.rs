//#![allow(unused)]

mod for_multi;
mod messenger;
mod renderer;
mod shaders;
mod window;

use std::{sync::mpsc, thread, time::Duration};

use messenger::{window_interface::InterfaceMessenger, window_renderer::{RendererMessenger, WindowMessenger}};
use renderer::Renderer;
use window::{Application};

fn main() {
    let (initial_sender, initial_receiver) = mpsc::channel();
    let (run_sender, run_receiver) = mpsc::channel();

    let renderer_handle = thread::spawn(move || {
        Renderer::new(WindowMessenger { initial_receiver, run_receiver}).run();
    });

    Application::new(
        RendererMessenger {initial_sender, run_sender},
        InterfaceMessenger {},
    )
    .run();
    renderer_handle.join().unwrap();
}
