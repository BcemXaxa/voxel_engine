use std::sync::Arc;

use vulkano::{
    device::{physical::PhysicalDevice, Device, Queue}, instance::Instance, pipeline::GraphicsPipeline, render_pass::Framebuffer, swapchain::{Swapchain, SwapchainCreateInfo}, Validated, VulkanError
};

use crate::messenger::window_renderer::WindowMessenger;

mod initialization;
mod swapchain;
mod drawing;
mod pipeline;

pub struct Renderer {
    window_msg: WindowMessenger,

    instance: Arc<Instance>,
    physical_device: Arc<PhysicalDevice>,
    device: Arc<Device>,
    queues: Vec<Arc<Queue>>,
    swapchain: Option<Arc<Swapchain>>,
    framebuffers: Option<Vec<Arc<Framebuffer>>>,
    graphics_pipeline: Option<Arc<GraphicsPipeline>>
}

impl Renderer {
    pub fn run(&mut self) {
        loop {
            match self.window_msg
        }
    }
}
