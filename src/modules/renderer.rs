pub mod initialization;
pub mod allocations;
mod drawing;
mod command_buffer;
mod swapchain;
pub mod queue;
mod render_pass;
mod physical_device;
mod logical_device;
mod pipeline;

use std::sync::Arc;

use queue::Queues;
use vulkano::{
    device::{physical::PhysicalDevice, Device}, image::{view::ImageView, Image}, instance::Instance, swapchain::Swapchain
};

pub struct Renderer {
    instance: Arc<Instance>,
    physical_device: Arc<PhysicalDevice>,
    device: Arc<Device>,
    queues: Queues,

    swapchain: Arc<Swapchain>,
    swapchain_images: Vec<(Arc<Image>, Arc<ImageView>)>,
}
