#![allow(unused)]

mod modules;

use std::{sync::mpsc, thread, time::Duration};

use modules::renderer::Renderer;
use modules::window::Application;

fn main() {
    Application::new().run();
}
