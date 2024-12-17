#![allow(unused)]

mod for_multi;
mod renderer;
mod shaders;
mod window;

use std::{sync::mpsc, thread, time::Duration};

use renderer::Renderer;
use window::{Application};

fn main() {

    Application::new()
    .run();
}