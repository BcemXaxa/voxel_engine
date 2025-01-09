pub mod allocations;
mod command_buffer;
mod drawing;
pub mod initialization;
mod logical_device;
mod physical_device;
mod pipeline;
pub mod queue;
mod render_pass;
mod swapchain;

use std::sync::Arc;

use queue::Queues;
use vulkano::{
    device::{physical::PhysicalDevice, Device},
    image::{view::ImageView, Image},
    instance::Instance,
    swapchain::Swapchain,
};

pub struct Renderer {
    instance: Arc<Instance>,
    physical_device: Arc<PhysicalDevice>,
    device: Arc<Device>,
    queues: Queues,

    swapchain: Arc<Swapchain>,
    swapchain_images: Vec<(Arc<Image>, Arc<ImageView>)>,
}
