#![allow(unused)]

mod for_multi;
mod messenger;
mod renderer;
mod shaders;
mod window;

use std::{sync::mpsc, thread, time::Duration};

use messenger::window_renderer::WindowMessenger;
use renderer::Renderer;
use window::{Application, InterfaceManager, RenderManager};

fn main() {
    let (initial_sender, initial_receiver) = mpsc::channel();

    let renderer_handle = thread::spawn(move || {
        Renderer::new(WindowMessenger { initial_receiver });
    });

    Application::new(
        RenderManager {initial_sender},
        InterfaceManager {},
    )
    .run();
}
