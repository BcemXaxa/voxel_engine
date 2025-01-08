use std::sync::Arc;

use vulkano::{
    device::{physical::PhysicalDevice, Device},
    format::Format,
    image::{
        view::{ImageView, ImageViewCreateInfo},
        Image, ImageUsage,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass},
    swapchain::{
        ColorSpace, PresentMode, Surface, SurfaceCapabilities, SurfaceInfo, Swapchain,
        SwapchainCreateInfo,
    },
    sync::Sharing,
};

use super::Renderer;

impl Renderer {
    pub fn swapchain_extent(&self) -> [u32; 2] {
        self.swapchain.image_extent()
    }

    pub fn recreate_swapchain(&mut self, new_extent: [u32; 2]) {
        if new_extent == self.swapchain_extent() {
            return
        }
        let (new_swapchain, images) = self
            .swapchain
            .recreate(SwapchainCreateInfo {
                image_extent: new_extent,
                ..self.swapchain.create_info()
            })
            .unwrap(); // TODO(handle_error)
        self.swapchain = new_swapchain;
        self.swapchain_images = Self::zip_image_views(images);
    }

    pub(super) fn create_swapchain(
        device: Arc<Device>,
        surface: Arc<Surface>,
    ) -> (Arc<Swapchain>, Vec<Arc<Image>>) {
        let SurfaceProperties {
            image_format,
            image_color_space,
            present_mode,
            image_extent,
            capabilities,
        } = Self::surface_properties(device.physical_device().clone(), surface.clone());

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
    }

    pub(super) fn zip_image_views(images: Vec<Arc<Image>>) -> Vec<(Arc<Image>, Arc<ImageView>)> {
        images
            .into_iter()
            .map(|image| {
                // TODO: possibly incorrect
                let create_info = ImageViewCreateInfo::from_image(&image);
                // TODO: handle error
                let image_view = ImageView::new(image.clone(), create_info).unwrap();
                (image, image_view)
            })
            .collect()
    }

    pub fn create_framebuffer(&self, image_i: u32, render_pass: Arc<RenderPass>) -> Arc<Framebuffer> {
        // TODO(optimize): framebuffer creation
        // this implementation means we create framebuffer for each render pass every frame
        // which (I guess) is not performant
        let create_info = FramebufferCreateInfo {
            attachments: vec![self.swapchain_images.get(image_i as usize).unwrap().1.clone()],
            extent: self.swapchain.image_extent(),
            ..Default::default()
        };
        Framebuffer::new(render_pass.clone(), create_info).unwrap() // TODO: handle error
    }

    fn surface_properties(
        physical_device: Arc<PhysicalDevice>,
        surface: Arc<Surface>,
    ) -> SurfaceProperties {
        let surface_info = SurfaceInfo {
            present_mode: Some(PresentMode::Mailbox),
            ..Default::default()
        };

        let capabilities = physical_device
            .surface_capabilities(&surface, surface_info.clone())
            .unwrap(); // TODO: handle error

        let formats = physical_device
            .surface_formats(&surface, surface_info)
            .unwrap(); // TODO: handle error

        // TODO: choose best option for image_format and image_color_space
        let (format, color_space) = formats
            .into_iter()
            .find(|(format, colorspace)| {
                *format == Format::B8G8R8A8_SRGB && *colorspace == ColorSpace::SrgbNonLinear
            })
            .unwrap();

        // TODO: choose best option for present mode
        let present_mode = capabilities
            .compatible_present_modes
            .iter()
            .next()
            .unwrap()
            .to_owned();

        // in fact, it is always capabilities.current_extent
        let extent = capabilities.current_extent.unwrap();

        SurfaceProperties {
            image_format: format,
            image_color_space: color_space,
            present_mode,
            image_extent: extent,
            capabilities,
        }
    }
}

#[derive(Clone)]
struct SurfaceProperties {
    pub image_format: Format,
    pub image_color_space: ColorSpace,
    pub present_mode: PresentMode,
    pub image_extent: [u32; 2],
    pub capabilities: SurfaceCapabilities,
}
