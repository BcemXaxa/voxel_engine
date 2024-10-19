#![allow(unused)]
mod for_multi;
mod vk_application;

use ash::{
    self,
    vk::{self},
};
use egui::ViewportBuilder;
use gpu_allocator::{vulkan::*, MemoryLocation};
use vk_application::*;

fn main() {
    egui_ash::run(
        "my_app",
        MyAppCreator,
        egui_ash::RunOption { 
            viewport_builder: Some(ViewportBuilder::default().with_title("some_title")),
            ..Default::default()
        }
    );
}
