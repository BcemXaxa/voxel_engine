use vulkano::{swapchain, VulkanError};

use super::Renderer;

impl Renderer{
    fn draw_frame(&mut self) {
        let acquire = swapchain::acquire_next_image(self.swapchain.clone(), None);

        let (image_i, _, acquire_future) = match acquire.map_err(Validated::unwrap) {
            Ok((_, true, _)) | Err(VulkanError::OutOfDate) => {
                self.recreate_swapchain([100; 2]); // FIXME: hardcoded values
                return;
            }
            Ok(ok) => ok,
            _ => panic!("Image acquisition failed"),
        };

        todo!()
    }
}