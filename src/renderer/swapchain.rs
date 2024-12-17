use std::sync::Arc;

use vulkano::{
    device::Device,
    image::{
        view::{ImageView, ImageViewCreateInfo},
        Image, ImageUsage,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass},
    swapchain::{
        Surface, Swapchain, SwapchainCreateInfo,
    },
    sync::Sharing,
};

use super::{initialization::SurfaceProperties, Renderer};

impl Renderer {
    pub(super) fn recreate_swapchain(&mut self, extent: [u32; 2]) {
        if self.swapchain.image_extent() != extent {
            let (new_swapchain, images) = self
                .swapchain
                .recreate(SwapchainCreateInfo {
                    image_extent: extent,
                    ..self.swapchain.create_info()
                })
                .unwrap(); // TODO: handle error
            let new_framebuffers =
                Self::create_framebuffers(new_swapchain.clone(), images, self.render_pass.clone());
            self.swapchain = new_swapchain;
            self.framebuffers = new_framebuffers;
        }
    }

    pub(super) fn create_swapchain(
        device: Arc<Device>,
        surface: Arc<Surface>,
        render_pass: Arc<RenderPass>,
        surface_properties: SurfaceProperties,
    ) -> (Arc<Swapchain>, Vec<Arc<Framebuffer>>) {
        let (swapchain, images) = {
            let SurfaceProperties {
                image_format,
                image_color_space,
                present_mode,
                image_extent,
                capabilities,
            } = surface_properties;

            // TODO: check for present mode suitability
            // TODO: check for format suitability
            // TODO: check for extent suitability

            // TODO: maybe add scaling behaviour and fullscreen
            let min_image_count = (capabilities.min_image_count + 1).max(3); // TODO: add some logic to adjust this value

            let create_info = SwapchainCreateInfo {
                present_mode,
                min_image_count,
                image_format,
                image_color_space,
                image_extent,
                image_usage: ImageUsage::COLOR_ATTACHMENT,
                image_sharing: Sharing::Exclusive,
                pre_transform: capabilities.current_transform,
                ..Default::default()
            };

            Swapchain::new(device, surface, create_info).unwrap()
        };

        let framebuffers = Self::create_framebuffers(swapchain.clone(), images, render_pass);
        (swapchain, framebuffers)
    }
    
    fn get_image_views(images: Vec<Arc<Image>>) -> Vec<Arc<ImageView>> {
        images
            .into_iter()
            .map(|image| {
                // TODO: possibly incorrect
                let create_info = ImageViewCreateInfo::from_image(&image);

                // TODO: handle error
                ImageView::new(image, create_info).unwrap()
            })
            .collect()
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
