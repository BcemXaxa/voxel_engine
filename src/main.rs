#![allow(unused)]

mod renderer;
mod shaders;
mod window;
mod math;
mod logic;
mod utility;

use std::{sync::mpsc, thread, time::Duration};

use renderer::Renderer;
use window::{Application};

fn main() {
    Application::new()
    .run();
}