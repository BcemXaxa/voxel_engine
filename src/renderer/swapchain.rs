use std::sync::Arc;

use vulkano::{
    device::{physical::PhysicalDevice, Device},
    format::Format,
    image::{Image, ImageUsage},
    swapchain::{
        self, ColorSpace, PresentMode, Surface, SurfaceCapabilities, Swapchain, SwapchainCreateInfo,
    },
    sync::Sharing,
};

use super::Renderer;

impl Renderer {
    
    fn create_swapchain(
        &mut self,
        physical_device: Arc<PhysicalDevice>,
        surface: Arc<Surface>,
        device: Arc<Device>,
    ) {
        let (image_format, image_color_space, present_mode, image_extent, capabilities) =
            Self::surface_properties(physical_device.clone(), surface.clone());

        let (new_swapchain, new_images) = {
            if let Some(swapchain) = self.swapchain.take() {
                swapchain
                    .recreate(SwapchainCreateInfo {
                        image_extent,
                        ..swapchain.create_info()
                    })
                    .unwrap() // TODO: handle error
            } else {
                // TODO: check for present mode suitability
                // TODO: check for format suitability
                // TODO: check for extent suitability

                // TODO: maybe add scaling behaviour and fullscreen
                let create_info = Self::create_info(
                    present_mode,
                    image_format,
                    image_color_space,
                    image_extent,
                    capabilities,
                );

                Swapchain::new(device, surface, create_info).unwrap()
            }
        };
    }

    fn create_info(
        present_mode: PresentMode,
        image_format: Format,
        image_color_space: ColorSpace,
        image_extent: [u32; 2],
        capabilities: SurfaceCapabilities,
    ) -> SwapchainCreateInfo {
        let min_image_count = (capabilities.min_image_count + 1).max(3); // TODO: add some logic to adjust this value

        SwapchainCreateInfo {
            present_mode,
            min_image_count,
            image_format,
            image_color_space,
            image_extent,
            image_usage: ImageUsage::COLOR_ATTACHMENT,
            image_sharing: Sharing::Exclusive, // FIXME: might not work because graphics_queue != present_queue
            pre_transform: capabilities.current_transform,
            ..Default::default()
        }
    }

    fn create_framebuffers(
        swapchain: Arc<Swapchain>,
        images: Vec<Arc<Image>>,
        render_pass: Arc<RenderPass>,
    ) -> Vec<Arc<Framebuffer>> {
        let image_views = Self::get_image_views(images);
        image_views
            .into_iter()
            .map(|image_view| {
                let create_info = FramebufferCreateInfo {
                    attachments: vec![image_view],
                    extent: swapchain.image_extent(),
                    ..Default::default()
                };
                Framebuffer::new(render_pass.clone(), create_info).unwrap() // TODO: handle error
            })
            .collect()
    }
}
