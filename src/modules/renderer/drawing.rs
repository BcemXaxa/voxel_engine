use vulkano::{
    command_buffer,
    swapchain::{self, SwapchainPresentInfo},
    sync::{self, GpuFuture},
    Validated, VulkanError,
};

use super::Renderer;

impl Renderer {
    pub(super) fn draw_frame(&self) -> Result<(), DrawError> {
        let acquire = swapchain::acquire_next_image(self.swapchain.clone(), None);

        let (image_i, _, acquire_future) = match acquire.map_err(Validated::unwrap) {
            Ok((_, true, _)) | Err(VulkanError::OutOfDate) => {
                return Err(DrawError::RecreationRequired);
            }
            Ok(ok) => ok,
            _ => return Err(DrawError::AcquisitionFailed),
        };

        let (command_buffer, queue) = self.command_buffers[image_i as usize].clone();

        let execution = sync::now(self.device.clone())
            .join(acquire_future)
            .then_execute(queue.clone(), command_buffer)
            .unwrap()
            .then_swapchain_present(
                queue.clone(),
                SwapchainPresentInfo::swapchain_image_index(self.swapchain.clone(), image_i),
            )
            .then_signal_fence_and_flush();

        match execution.map_err(Validated::unwrap) {
            Ok(future) => future.wait(None).unwrap(),
            Err(VulkanError::OutOfDate) => return Err(DrawError::RecreationRequired),
            Err(_) => return Err(DrawError::ExecutionFailed),
        }
        Ok(())
    }
}

pub(super) enum DrawError {
    RecreationRequired,
    AcquisitionFailed,
    ExecutionFailed,
}
