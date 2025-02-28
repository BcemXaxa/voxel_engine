use std::sync::Arc;

use vulkano::{
    command_buffer::PrimaryAutoCommandBuffer,
    image::view::ImageView,
    render_pass::{Framebuffer, RenderPass},
    swapchain::{self, SwapchainPresentInfo},
    sync::{self, GpuFuture},
    Validated, VulkanError,
};

use super::{queue::QueueType, Renderer};

impl Renderer {
    pub fn execute_then_present<F>(
        &self,
        render_passes: Vec<(Arc<RenderPass>, Option<Arc<ImageView>>)>,
        command_buffer: F,
    ) -> Result<(), DrawError>
    where
        F: FnOnce(Vec<Arc<Framebuffer>>) -> Arc<PrimaryAutoCommandBuffer>,
    {
        let acquire = swapchain::acquire_next_image(self.swapchain.clone(), None);

        let (image_i, _, acquire_future) = match acquire.map_err(Validated::unwrap) {
            Ok((_, true, _)) | Err(VulkanError::OutOfDate) => {
                return Err(DrawError::RecreationRequired);
            }
            Ok(ok) => ok,
            _ => return Err(DrawError::AcquisitionFailed),
        };

        let framebuffers = render_passes
            .into_iter()
            .map(|(render_pass, depth_image)| {
                self.create_framebuffer(image_i, render_pass, depth_image)
            })
            .collect();

        let queue = self.queues.get(QueueType::GraphicsPresent).unwrap();
        let execution = sync::now(self.device.clone())
            .join(acquire_future)
            .then_execute(queue.clone(), command_buffer(framebuffers))
            .unwrap()
            .then_swapchain_present(
                queue,
                SwapchainPresentInfo::swapchain_image_index(self.swapchain.clone(), image_i),
            )
            .then_signal_fence_and_flush();

        match execution.map_err(Validated::unwrap) {
            // TODO: probably should not wait
            Ok(future) => future.wait(None).unwrap(),
            Err(VulkanError::OutOfDate) => return Err(DrawError::RecreationRequired),
            Err(_) => return Err(DrawError::ExecutionFailed),
        }
        Ok(())
    }
}

pub enum DrawError {
    RecreationRequired,
    AcquisitionFailed,
    ExecutionFailed,
}
