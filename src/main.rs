#![allow(unused)]
mod for_multi;
mod vk_application;

use vk_application::App;
use winit;
use egui_winit_vulkano;

fn main() {
    let app = App::new();
}
