use std::{
    collections::HashSet,
    sync::{Arc, LazyLock},
};

use vulkano::{
    command_buffer::{
        allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo},
        AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer,
        RenderPassBeginInfo, SubpassBeginInfo, SubpassEndInfo,
    },
    device::{
        physical::{PhysicalDevice, PhysicalDeviceType},
        Device, DeviceCreateInfo, DeviceExtensions, Features, Queue, QueueCreateInfo, QueueFlags,
    },
    format::Format,
    image::{
        sampler::ComponentMapping,
        view::{ImageView, ImageViewCreateInfo, ImageViewType},
        Image, ImageLayout, ImageSubresourceRange, ImageUsage, SampleCount,
    },
    instance::{Instance, InstanceCreateInfo, InstanceExtensions},
    pipeline::{
        graphics::{
            color_blend::{
                AttachmentBlend, BlendFactor, BlendOp, ColorBlendAttachmentState, ColorBlendState,
                ColorComponents,
            },
            input_assembly::{InputAssemblyState, PrimitiveTopology},
            multisample::MultisampleState,
            rasterization::{
                CullMode, FrontFace, LineRasterizationMode, PolygonMode, RasterizationState,
            },
            subpass::PipelineSubpassType,
            vertex_input::VertexInputState,
            viewport::{Scissor, Viewport, ViewportState},
            GraphicsPipelineCreateInfo,
        },
        layout::PipelineLayoutCreateInfo,
        DynamicState, GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo,
    },
    render_pass::{
        AttachmentDescription, AttachmentLoadOp, AttachmentReference, AttachmentStoreOp,
        Framebuffer, FramebufferCreateInfo, RenderPass, RenderPassCreateInfo, SubpassDescription,
    },
    swapchain::{
        ColorSpace, PresentMode, Surface, SurfaceCapabilities, SurfaceInfo, Swapchain,
        SwapchainCreateInfo,
    },
    sync::{semaphore::Semaphore, Sharing},
    Version, VulkanLibrary,
};

use crate::messenger::window_renderer::WindowMessenger;

use super::Renderer;
use crate::shaders;

// FIXME: possible memory leak
// static members don't call "Drop" on program termination,
// so this may cause memory leak, but it's ok as long as OS takes care of it
static LIBRARY: LazyLock<Arc<VulkanLibrary>> =
    LazyLock::new(|| VulkanLibrary::new().expect("Vulkan library is not supported"));

impl Renderer {
    pub fn new(window_msg: WindowMessenger) -> Self {
        let (window, extensions) = window_msg.initial_receiver.recv().unwrap();

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

        let render_pass = Self::create_render_pass(device.clone(), surface_properties.0);

        Self {
            instance,
            physical_device,
            device,
            queues,
            swapchain: None,
            framebuffers: None,
            graphics_pipeline: None,
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
            .filter(|physical_device| {
                Self::is_physical_device_suitable(
                    physical_device.clone(),
                )
            })
            .collect();

        physical_devices
            .into_iter()
            .next()
            .expect("No suitable physical devices found")
    }

    fn is_physical_device_suitable(
        physical_device: Arc<PhysicalDevice>,
    ) -> bool {
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
    ) -> (Arc<Device>, Vec<Arc<Queue>>) {
        let queue_families = physical_device.queue_family_properties();

        let graphics_queue_family = queue_families
            .iter()
            .enumerate()
            .position(|(_, family_properties)| {
                family_properties.queue_flags.contains(QueueFlags::GRAPHICS)
            })
            .expect("Logical device creation failed: No graphics queues found")
            as u32;

        let present_queue_family = queue_families
            .iter()
            .enumerate()
            .position(|(queue_family_index, _)| {
                let queue_family_index = queue_family_index as u32;
                physical_device
                    .surface_support(queue_family_index, &surface)
                    .unwrap()
            })
            .expect("Logical device creation failed: No presentation queues found")
            as u32;

        let transfer_queue_family = queue_families
            .iter()
            .enumerate()
            .position(|(_, family_properties)| {
                family_properties.queue_flags.contains(QueueFlags::TRANSFER)
            })
            .expect("Logical device creation failed: No transfer queues found")
            as u32;

        let queue_indices = HashSet::from([
            graphics_queue_family,
            present_queue_family,
            transfer_queue_family,
        ]);

        let queue_create_infos: Vec<QueueCreateInfo> = queue_indices
            .into_iter()
            .map(|queue_family_index| QueueCreateInfo {
                queue_family_index,
                queues: vec![1.0],
                ..Default::default()
            })
            .collect();

        let enabled_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..Default::default()
        };

        let create_info = DeviceCreateInfo {
            queue_create_infos,
            enabled_extensions,
            enabled_features: Features::empty(),
            ..Default::default()
        };

        let (device, queues) =
            Device::new(physical_device, create_info).expect("Logical device creation failed");
        (device, queues.collect())
    }

    fn surface_info() -> SurfaceInfo {
        SurfaceInfo {
            present_mode: Some(PresentMode::Mailbox),
            ..Default::default()
        }
    }

    pub(super) fn surface_properties (
        physical_device: Arc<PhysicalDevice>,
        surface: Arc<Surface>,
    ) -> (Format, ColorSpace, PresentMode, [u32; 2], SurfaceCapabilities) {
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
        let image_extent = capabilities.current_extent.unwrap();

        (format, color_space, present_mode, image_extent, capabilities)
    }

    
    fn get_image_views(images: Vec<Arc<Image>>) -> Vec<Arc<ImageView>> {
        images
            .into_iter()
            .map(|image| {
                let create_info = ImageViewCreateInfo {
                    view_type: ImageViewType::Dim2d,
                    format: image.format(),
                    usage: image.usage(),
                    component_mapping: ComponentMapping::identity(),
                    subresource_range: ImageSubresourceRange::from_parameters(
                        image.format(),
                        image.mip_levels(),
                        image.array_layers(),
                    ),
                    ..Default::default()
                };

                // TODO: handle error
                ImageView::new(image, create_info).unwrap()
            })
            .collect()
    }


    fn create_render_pass(device: Arc<Device>, swapchain_image_format: Format) -> Arc<RenderPass> {
        let attachment_description = AttachmentDescription {
            format: swapchain_image_format,
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

    fn create_command_buffer_builder(
        allocator: &StandardCommandBufferAllocator,
        queue: Arc<Queue>,
    ) -> CmdBuilder {
        AutoCommandBufferBuilder::primary(
            allocator,
            queue.queue_family_index(),
            CommandBufferUsage::MultipleSubmit,
        )
        .unwrap() // TODO: handle error
    }

    fn write_command_buffer(
        mut cmd_builder: CmdBuilder,
        framebuffer: Arc<Framebuffer>,
        graphics_pipeline: Arc<GraphicsPipeline>,
    ) -> Arc<PrimaryAutoCommandBuffer> {
        // TODO: handle errors
        cmd_builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![Some([0.0, 0.0, 0.0, 1.0].into())], // TODO: make logic to handle possible framebuffer attachments
                    ..RenderPassBeginInfo::framebuffer(framebuffer)
                },
                SubpassBeginInfo::default(),
            )
            .unwrap()
            .bind_pipeline_graphics(graphics_pipeline)
            .unwrap()
            // TODO: set viewport if neccessary
            // TODO: set scissors if neccessary
            .draw(3, 1, 0, 0) // FIXME: hardcoded
            .unwrap()
            .end_render_pass(SubpassEndInfo::default())
            .unwrap();

        cmd_builder.build().unwrap() // TODO: handle error
    }
    fn create_sync_objects(device: Arc<Device>) -> (Semaphore, Semaphore) {
        let image_available_semaphore = Semaphore::from_pool(device.clone()).unwrap(); // TODO: handle error
        let render_finished_semaphore = Semaphore::from_pool(device.clone()).unwrap(); // TODO: handle error

        (image_available_semaphore, render_finished_semaphore)
    }
}

type CmdBuilder =
    AutoCommandBufferBuilder<PrimaryAutoCommandBuffer<StandardCommandBufferAllocator>>;
