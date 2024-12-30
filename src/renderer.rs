use std::sync::{mpsc::TryRecvError, Arc};

use drawing::DrawError;
use queue::Queues;
use vertex_buffer::MyVertex;
use vulkano::{
    buffer::Subbuffer, command_buffer::{allocator::StandardCommandBufferAllocator, PrimaryAutoCommandBuffer}, device::{physical::PhysicalDevice, Device, Queue}, instance::Instance, memory::allocator::StandardMemoryAllocator, pipeline::GraphicsPipeline, render_pass::{Framebuffer, RenderPass}, swapchain::Swapchain
};
use winit::window::Window;

mod command_buffer;
mod drawing;
mod initialization;
mod pipeline;
mod swapchain;
mod queue;
mod render_pass;
mod physical_device;
mod logical_device;
mod vertex_buffer;

pub struct Renderer {
    window: Arc<Window>,

    instance: Arc<Instance>,
    physical_device: Arc<PhysicalDevice>,
    device: Arc<Device>,
    queues: Queues,

    render_pass: Arc<RenderPass>,
    swapchain: Arc<Swapchain>,
    framebuffers: Vec<Arc<Framebuffer>>,
    graphics_pipeline: Arc<GraphicsPipeline>,

    command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    command_buffers: Vec<(Arc<PrimaryAutoCommandBuffer>, Arc<Queue>)>,

    memory_allocator: Arc<StandardMemoryAllocator>,
    vertex_buffer: Subbuffer<[MyVertex]>,
}

impl Renderer {
    pub fn draw(&mut self) {
        match self.draw_frame() {
            Ok(_) => {},
            Err(err) => match err {
                DrawError::RecreationRequired => println!("recreation"),
                DrawError::AcquisitionFailed => println!("acquisiton"),
                DrawError::ExecutionFailed => println!("execution"),
            },
        }
    }

    pub fn recreate(&mut self, extent: Option<[u32; 2]>) {
        let extent = match extent {
            Some(val) => val,
            None => self.window.inner_size().into(),
        };
        self.recreate_swapchain(extent);
        self.recreate_graphics_pipeline(extent);
        self.recreate_command_buffers();
    }
}

pub enum RendererIncomingEvent {
    ExtentChange([u32; 2]),
}
