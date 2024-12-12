use std::sync::{mpsc::TryRecvError, Arc};

use drawing::DrawError;
use initialization::Queues;
use vulkano::{
    command_buffer::{allocator::StandardCommandBufferAllocator, PrimaryAutoCommandBuffer}, device::{physical::PhysicalDevice, Device, Queue}, instance::Instance, pipeline::GraphicsPipeline, render_pass::{Framebuffer, RenderPass}, swapchain::Swapchain
};
use winit::window::Window;

use crate::messenger::window_renderer::WindowMessenger;

mod initialization;
mod swapchain;
mod pipeline;
mod command_buffer;
mod drawing;

pub struct Renderer {
    window_msg: WindowMessenger,
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
}

impl Renderer {
    pub fn run(&mut self) {
        loop {
            match self.window_msg.run_receiver.try_recv() {
                Err(err) => match err {
                    TryRecvError::Empty => {
                        if let Err(err) = self.draw_frame() {
                            match err {
                                DrawError::RecreationRequired => self.recreate(None),
                                DrawError::AcquisitionFailed => panic!("Image acquisition failed"),
                                DrawError::ExecutionFailed => panic!("Execution failed")
                            }
                        }
                    }
                    TryRecvError::Disconnected => break,
                },
                Ok(event) => match event {
                    RendererIncomingEvent::ExtentChange(extent) => {
                        self.recreate(Some(extent));
                    }
                },
            }
        }
    }

    fn recreate(&mut self, extent: Option<[u32; 2]>) {
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
