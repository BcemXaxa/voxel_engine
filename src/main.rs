#![allow(unused)]

use std::{sync::mpsc, thread};

use renderer::Renderer;
use window::{Application, InterfaceManager, RenderManager};
mod for_multi;
mod renderer;
mod window;

fn main() {
    let (required_extensions_sender, required_extensions_receiver) = mpsc::channel();
    let (window_sender, window_receiver) = mpsc::channel();

    let renderer_handle = thread::spawn(move || {
        Renderer::new(required_extensions_receiver, window_receiver);
    });

    Application::new(
        RenderManager {
            required_extensions_sender,
            window_sender,
        },
        InterfaceManager {},
    )
    .run();
}
