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

    fn new_physical_device(instance: Arc<Instance>, surface: Arc<Surface>) -> Arc<PhysicalDevice> {
        let physical_devices: Vec<_> = instance
            .enumerate_physical_devices()
            .expect("Physical devices enumeration failed")
            .filter(|physical_device| Self::is_physical_device_suitable(physical_device.clone()))
            .collect();

        physical_devices
            .into_iter()
            .next()
            .expect("No suitable physical devices found")
    }

    fn is_physical_device_suitable(physical_device: Arc<PhysicalDevice>) -> bool {
        let properties = physical_device.properties();
        let mut has_properties = true;
        has_properties &= properties.device_type == PhysicalDeviceType::DiscreteGpu;

        let features = physical_device.supported_features();
        let mut has_features = true;
        has_features &= features.geometry_shader;

        let extensions = physical_device.supported_extensions();
        let mut has_extensions = true;
        has_extensions &= extensions.khr_swapchain;

        // TODO: make score system (optional)
        // TODO: make list of missing properties & features & extensions (optional)

        has_properties && has_features && has_extensions
    }

    fn create_logical_device(
        physical_device: Arc<PhysicalDevice>,
        surface: Arc<Surface>,
    ) -> (Arc<Device>, Queues) {
        let families_properties = physical_device.queue_family_properties();

        let queue_indices =
            families_properties
                .iter()
                .enumerate()
                .filter_map(|(index, family_properties)| {
                    let index = index as u32;
                    if family_properties.queue_flags.intersects(
                        QueueFlags::GRAPHICS | QueueFlags::COMPUTE | QueueFlags::TRANSFER,
                    ) {
                        Some(index)
                    } else if physical_device.surface_support(index, &surface).unwrap() {
                        Some(index)
                    } else {
                        None
                    }
                });

        let queue_create_infos: Vec<QueueCreateInfo> = queue_indices
            .map(|queue_family_index| QueueCreateInfo {
                queue_family_index,
                queues: vec![1.0],
                ..Default::default()
            })
            .collect();

        let enabled_extensions = DeviceExtensions {
            khr_swapchain: true, // TODO: move extension?
            ..Default::default()
        };

        let create_info = DeviceCreateInfo {
            queue_create_infos,
            enabled_extensions,
            enabled_features: Features::empty(),
            ..Default::default()
        };

        let (device, queues) = Device::new(physical_device.clone(), create_info)
            .expect("Logical device creation failed");
        let queues = Queues::new(
            queues.collect(),
            &families_properties,
            physical_device.clone(),
            &surface,
        );
        (device, queues)
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

    fn create_render_pass(device: Arc<Device>, image_format: Format) -> Arc<RenderPass> {
        let attachment_description = AttachmentDescription {
            format: image_format,
            samples: SampleCount::Sample1,
            load_op: AttachmentLoadOp::Clear,
            store_op: AttachmentStoreOp::Store,
            initial_layout: ImageLayout::Undefined,
            final_layout: ImageLayout::PresentSrc,
            ..Default::default()
        };

        let attachment_reference = AttachmentReference {
            attachment: 0,
            layout: ImageLayout::ColorAttachmentOptimal,
            ..Default::default()
        };

        let subpass_description = SubpassDescription {
            color_attachments: vec![Some(attachment_reference)],
            ..Default::default()
        };

        RenderPass::new(
            device.clone(),
            RenderPassCreateInfo {
                attachments: vec![attachment_description],
                subpasses: vec![subpass_description],
                ..Default::default()
            },
        )
        .unwrap() // TODO: handle error
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

type PresentSupport = bool;
pub(super) struct Queues {
    graphics_queues: Vec<(Arc<Queue>, QueueFlags, PresentSupport)>,
    compute_queues: Vec<(Arc<Queue>, QueueFlags, PresentSupport)>,
    transfer_queues: Vec<(Arc<Queue>, QueueFlags, PresentSupport)>,
}

impl Queues {
    fn new(
        queues: Vec<Arc<Queue>>,
        families_properties: &[QueueFamilyProperties],
        device: Arc<PhysicalDevice>,
        surface: &Surface,
    ) -> Self {
        let mut graphics_queues = Vec::new();
        let mut compute_queues = Vec::new();
        let mut transfer_queues = Vec::new();

        for queue in queues.into_iter() {
            let family_index = queue.queue_family_index();
            let flags = families_properties[family_index as usize].queue_flags;
            let present_support = device.surface_support(family_index, surface).unwrap();

            if flags.contains(QueueFlags::GRAPHICS) {
                graphics_queues.push((queue, flags, present_support));
            } else if flags.contains(QueueFlags::GRAPHICS) {
                compute_queues.push((queue, flags, present_support));
            } else if flags.contains(QueueFlags::GRAPHICS) {
                transfer_queues.push((queue, flags, present_support));
            }
        }

        Self {
            graphics_queues,
            compute_queues,
            transfer_queues,
        }
    }

    pub(super) fn graphics(&self) -> Result<Arc<Queue>, &str> {
        match self.graphics_queues.iter().next() {
            Some((queue, _, _)) => Ok(queue.clone()),
            None => Err("Graphics queue was not found"),
        }
    }

    pub(super) fn graphics_present(&self) -> Result<Arc<Queue>, &str> {
        match self.graphics_queues.iter().find(|(_, _, present)| *present) {
            Some((queue, _, _)) => Ok(queue.clone()),
            None => Err("Graphics queue supporting presentation was not found"),
        }
    }

    pub(super) fn compute(&self) -> Result<Arc<Queue>, &str> {
        if let Some((queue, _, _)) = self.compute_queues.iter().next() {
            Ok(queue.clone())
        } else if let Ok(queue) = self.graphics() {
            Ok(queue)
        } else {
            Err("Compute queue was not found")
        }
    }

    pub(super) fn transfer(&self) -> Result<Arc<Queue>, &str> {
        if let Some((queue, _, _)) = self.transfer_queues.iter().next() {
            Ok(queue.clone())
        } else if let Ok(queue) = self.compute() {
            Ok(queue)
        } else {
            Err("Transfer queue was not found")
        }
    }
}
