use std::sync::{Arc, LazyLock};

use vulkano::{
    command_buffer::allocator::{
        StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
    },
    device::{
        physical::{PhysicalDevice, PhysicalDeviceType},
        Device, DeviceCreateInfo, DeviceExtensions, Features, Queue, QueueCreateInfo,
        QueueFamilyProperties, QueueFlags,
    },
    format::Format,
    image::{ImageLayout, SampleCount},
    instance::{Instance, InstanceCreateInfo, InstanceExtensions},
    render_pass::{
        AttachmentDescription, AttachmentLoadOp, AttachmentReference, AttachmentStoreOp,
        RenderPass, RenderPassCreateInfo, SubpassDescription,
    },
    swapchain::{ColorSpace, PresentMode, Surface, SurfaceCapabilities, SurfaceInfo},
    Version, VulkanLibrary,
};
use winit::window::{self, Window};

use super::Renderer;

// FIXME: possible memory leak
// static members don't call "Drop" on program termination,
// so this may cause memory leak, but it's ok as long as OS takes care of it
static LIBRARY: LazyLock<Arc<VulkanLibrary>> =
    LazyLock::new(|| VulkanLibrary::new().expect("Vulkan library is not supported"));

impl Renderer {
    pub fn new(window: Arc<Window>, extensions: InstanceExtensions) -> Self {

        let instance_extensions = InstanceExtensions {
            ext_surface_maintenance1: true,
            ..extensions
        };

        let instance = Self::new_instance(instance_extensions);
        let surface = Surface::from_window(instance.clone(), window.clone())
            .expect("Surface creation failed");

        let physical_device = Self::new_physical_device(instance.clone(), surface.clone());
        let (device, queues) =
            Self::create_logical_device(physical_device.clone(), surface.clone());

        let surface_properties = Self::surface_properties(physical_device.clone(), surface.clone());

        let render_pass = Self::create_render_pass(device.clone(), surface_properties.image_format);

        let (swapchain, framebuffers) = Self::create_swapchain(
            device.clone(),
            surface.clone(),
            render_pass.clone(),
            surface_properties.clone(),
        );
        let graphics_pipeline = Self::create_graphics_pipeline(
            device.clone(),
            render_pass.clone(),
            surface_properties.image_extent,
        );

        let command_buffer_allocator = Self::create_command_buffer_allocator(device.clone());
        let command_buffers = Self::write_command_buffers(
            &command_buffer_allocator,
            queues.graphics_present().unwrap(),
            framebuffers.clone(),
            graphics_pipeline.clone(),
        );

        Self {
            window,

            instance,
            physical_device,
            device,
            queues,

            render_pass,
            graphics_pipeline,
            swapchain,
            framebuffers,

            command_buffer_allocator,
            command_buffers,
        }
    }

    fn new_instance(enabled_extensions: InstanceExtensions) -> Arc<Instance> {
        let application_version = Version {
            major: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            minor: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
            patch: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
        };

        let engine_version = application_version;

        let create_info = InstanceCreateInfo {
            application_name: Some("VoxelEngine".to_string()),
            engine_name: Some("voxen".to_string()),
            application_version,
            engine_version,
            enabled_extensions,
            ..Default::default()
        };

        Instance::new(LIBRARY.clone(), create_info).expect("Instance creation failed")
    }

    fn surface_info() -> SurfaceInfo {
        SurfaceInfo {
            present_mode: Some(PresentMode::Mailbox),
            ..Default::default()
        }
    }

    pub(super) fn surface_properties(
        physical_device: Arc<PhysicalDevice>,
        surface: Arc<Surface>,
    ) -> SurfaceProperties {
        let capabilities = physical_device
            .surface_capabilities(&surface, Self::surface_info())
            .unwrap(); // TODO: handle error

        // TODO: choose best option for image_format and image_color_space
        let formats = physical_device
            .surface_formats(&surface, Self::surface_info())
            .unwrap(); // TODO: handle error

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

    fn create_command_buffer_allocator(device: Arc<Device>) -> Arc<StandardCommandBufferAllocator> {
        // TODO: adjust primary_buffer_count
        let create_info = StandardCommandBufferAllocatorCreateInfo::default();
        Arc::new(StandardCommandBufferAllocator::new(
            device.clone(),
            create_info,
        ))
    }
}

#[derive(Clone)]
pub(super) struct SurfaceProperties {
    pub image_format: Format,
    pub image_color_space: ColorSpace,
    pub present_mode: PresentMode,
    pub image_extent: [u32; 2],
    pub capabilities: SurfaceCapabilities,
}