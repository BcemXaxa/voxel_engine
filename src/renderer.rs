#![allow(unused)]
use std::{
    collections::HashSet,
    ops::RangeInclusive,
    sync::{mpsc::Receiver, Arc},
};

use vulkano::{
    command_buffer::{
        self, allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo}, AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer, RenderPassBeginInfo, SubpassBeginInfo, SubpassEndInfo
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
    sync::Sharing,
    Version, VulkanLibrary,
};
use winit::window::Window;

use crate::shaders;

pub struct Renderer {
    pub library: Arc<VulkanLibrary>,
    pub instance: Arc<Instance>,
    pub physical_device: Arc<PhysicalDevice>,
    pub device: Arc<Device>,
    pub queues: Vec<Arc<Queue>>,
    pub swapchain: Arc<Swapchain>,
}

impl Renderer {
    pub fn run() {}

    pub fn new(
        required_extensions_receiver: Receiver<InstanceExtensions>,
        window_receiver: Receiver<Arc<Window>>,
    ) -> Self {
        let library = VulkanLibrary::new().expect("Library creation failed");

        let enabled_extensions = InstanceExtensions {
            ext_surface_maintenance1: true,
            ..required_extensions_receiver.recv().unwrap()
        };
        let instance = Self::new_instance(library.clone(), enabled_extensions);
        let surface = Surface::from_window(instance.clone(), window_receiver.recv().unwrap())
            .expect("Surface creation failed");

        let physical_device = Self::new_physical_device(instance.clone(), &surface);
        let (device, queues) = Self::new_logical_device(physical_device.clone(), &surface);

        let (swapchain, images) =
            Self::get_swapchain(&physical_device, surface.clone(), device.clone());

        let image_views = images
            .iter()
            .map(|image| Self::get_image_view(image.clone()))
            .collect::<Vec<_>>();

        let render_pass = Self::create_render_pass(device.clone(), swapchain.image_format());
        let graphics_pipeline = Self::create_graphics_pipeline(
            device.clone(),
            render_pass.clone(),
            swapchain.image_extent(),
        );

        let framebuffers =
            Self::create_framebuffers(swapchain.clone(), image_views.clone(), render_pass.clone());

        Self {
            library,
            instance,
            physical_device,
            device,
            queues,
            swapchain,
        }
    }

    fn new_instance(
        library: Arc<VulkanLibrary>,
        enabled_extensions: InstanceExtensions,
    ) -> Arc<Instance> {
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

        Instance::new(library, create_info).expect("Instance creation failed")
    }

    fn new_physical_device(instance: Arc<Instance>, surface: &Surface) -> Arc<PhysicalDevice> {
        let physical_devices: Vec<_> = instance
            .enumerate_physical_devices()
            .expect("Physical devices enumeration failed")
            .filter(|physical_device| {
                Self::is_physical_device_suitable(physical_device, &instance, surface)
            })
            .collect();

        physical_devices
            .into_iter()
            .next()
            .expect("No suitable physical devices found")
    }

    fn is_physical_device_suitable(
        physical_device: &PhysicalDevice,
        instance: &Instance,
        surface: &Surface,
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

    fn new_logical_device(
        physical_device: Arc<PhysicalDevice>,
        surface: &Surface,
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
                    .surface_support(queue_family_index, surface)
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

    fn get_swapchain(
        physical_device: &PhysicalDevice,
        surface: Arc<Surface>,
        device: Arc<Device>,
    ) -> (Arc<Swapchain>, Vec<Arc<Image>>) {
        let surface_info = SurfaceInfo {
            present_mode: Some(PresentMode::Mailbox),
            ..Default::default()
        };
        // TODO: handle error
        let capabilities = physical_device
            .surface_capabilities(&surface, surface_info.clone())
            .unwrap();

        // TODO: choose best option for present mode
        let present_mode = capabilities
            .compatible_present_modes
            .iter()
            .next()
            .unwrap()
            .to_owned();

        // TODO: handle error
        let formats = physical_device
            .surface_formats(&surface, surface_info.clone())
            .unwrap();

        // TODO: choose best option for image_format and image_color_space
        let (image_format, image_color_space) = formats
            .into_iter()
            .find(|(format, colorspace)| {
                format == &Format::B8G8R8A8_SRGB && colorspace == &ColorSpace::SrgbNonLinear
            })
            .unwrap();

        let image_extent = Self::choose_swap_extent(&capabilities);

        // TODO: check for present mode suitability
        // TODO: check for format suitability
        // TODO: check for extent suitability

        let min_image_count = 3; // TODO: add some logic to adjust this value

        // TODO: maybe add scaling behaviour and fullscreen
        let create_info = SwapchainCreateInfo {
            present_mode,
            min_image_count,
            image_format,
            image_color_space,
            image_extent,
            image_usage: ImageUsage::COLOR_ATTACHMENT,
            image_sharing: Sharing::Exclusive, // FIXME: might not work because graphics_queue != present_queue
            pre_transform: capabilities.current_transform,
            ..Default::default()
        };

        Swapchain::new(device, surface, create_info).unwrap()
    }

    fn choose_swap_extent(capabilities: &SurfaceCapabilities) -> [u32; 2] {
        let min = capabilities.min_image_extent;
        let max = capabilities.max_image_extent;
        let width = 600.clamp(min[0], max[0]);
        let height = 600.clamp(min[1], max[1]);
        // in fact, it is always capabilities.current_extent
        [width, height] // FIXME: use actual values
    }

    fn get_image_view(image: Arc<Image>) -> Arc<ImageView> {
        let create_info = ImageViewCreateInfo {
            view_type: ImageViewType::Dim2d,
            format: image.format(),
            usage: image.usage(),
            component_mapping: ComponentMapping::identity(),
            subresource_range: ImageSubresourceRange::from_parameters(
                Format::B8G8R8A8_SRGB,
                image.mip_levels(),
                image.array_layers(),
            ),
            ..Default::default()
        };

        // TODO: handle error
        ImageView::new(image, create_info).unwrap()
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

    fn create_graphics_pipeline(
        device: Arc<Device>,
        render_pass: Arc<RenderPass>,
        swapchain_extent: [u32; 2],
    ) -> Arc<GraphicsPipeline> {
        // TODO: handle error
        let vertex_shader = shaders::default_vertex_shader::load(device.clone()).unwrap();
        // TODO: handle error
        let fragment_shader = shaders::default_fragment_shader::load(device.clone()).unwrap();

        let pipeline_stages = [
            {
                // TODO: handle error
                let entry_point = vertex_shader.entry_point("main").unwrap();
                PipelineShaderStageCreateInfo::new(entry_point)
            },
            {
                // TODO: handle error
                let entry_point = fragment_shader.entry_point("main").unwrap();
                PipelineShaderStageCreateInfo::new(entry_point)
            },
        ];

        let dynamic_states = [DynamicState::Viewport, DynamicState::Scissor];

        // TODO: real vertex input state
        let vertex_input_state = VertexInputState {
            ..Default::default()
        };

        let input_assembly_state = InputAssemblyState {
            topology: PrimitiveTopology::TriangleList,
            primitive_restart_enable: false,
            ..Default::default()
        };

        let viewport_state = {
            let viewport = Viewport {
                // FIXME: hardcoded?
                offset: [0.0, 0.0],
                extent: [swapchain_extent[0] as f32, swapchain_extent[1] as f32],
                depth_range: RangeInclusive::new(0.0, 1.0),
            };

            let scissor = Scissor {
                // FIXME: hardcoded?
                offset: [0, 0],
                extent: swapchain_extent,
            };

            ViewportState {
                viewports: [viewport].into(),
                scissors: [scissor].into(),
                ..Default::default()
            }
        };

        let rasterization_state = RasterizationState {
            depth_clamp_enable: false,
            rasterizer_discard_enable: false, // TODO: maybe change later
            polygon_mode: PolygonMode::Fill,
            cull_mode: CullMode::Back, // TODO: check if it is faster than other methods of backculling
            front_face: FrontFace::CounterClockwise,
            depth_bias: None,
            line_width: 1.0,
            line_rasterization_mode: LineRasterizationMode::Default,
            line_stipple: None,
            ..Default::default()
        };

        let multisample_state = MultisampleState {
            // TODO: use multisampling
            ..Default::default()
        };

        // TODO: maybe depth and stencil testing

        let color_blend_attachment_state = ColorBlendAttachmentState {
            blend: Some(AttachmentBlend {
                src_color_blend_factor: BlendFactor::One,
                dst_color_blend_factor: BlendFactor::Zero,
                color_blend_op: BlendOp::Add,
                src_alpha_blend_factor: BlendFactor::One,
                dst_alpha_blend_factor: BlendFactor::Zero,
                alpha_blend_op: BlendOp::Add,
            }),
            color_write_enable: true,
            color_write_mask: ColorComponents::all(),
        };

        let color_blend_state = ColorBlendState {
            attachments: vec![color_blend_attachment_state],
            logic_op: None,
            blend_constants: [0.0; 4],
            flags: Default::default(),
            ..Default::default()
        };

        // TODO: handle error
        let pipeline_layout =
            PipelineLayout::new(device.clone(), PipelineLayoutCreateInfo::default()).unwrap();

        {
            let mut create_info = GraphicsPipelineCreateInfo::layout(pipeline_layout);
            create_info.stages.extend(pipeline_stages);
            create_info.vertex_input_state = Some(vertex_input_state);
            create_info.input_assembly_state = Some(input_assembly_state);
            create_info.viewport_state = Some(viewport_state);
            create_info.rasterization_state = Some(rasterization_state);
            create_info.multisample_state = Some(multisample_state);
            create_info.color_blend_state = Some(color_blend_state);
            create_info.dynamic_state.extend(dynamic_states);
            create_info.subpass = Some(PipelineSubpassType::BeginRenderPass(
                render_pass.first_subpass(),
            ));
            create_info.base_pipeline = None;
            GraphicsPipeline::new(device.clone(), None, create_info)
        }
        .unwrap() // TODO: handle error
    }

    fn create_framebuffers(
        swapchain: Arc<Swapchain>,
        image_views: Vec<Arc<ImageView>>,
        render_pass: Arc<RenderPass>,
    ) -> Vec<Arc<Framebuffer>> {
        let mut framebuffers = Vec::new();

        for image_view in image_views {
            let create_info = FramebufferCreateInfo {
                attachments: vec![image_view],
                extent: swapchain.image_extent(),
                ..Default::default()
            };

            // TODO: handle error
            let framebuffer = Framebuffer::new(render_pass.clone(), create_info).unwrap();
            framebuffers.push(framebuffer);
        }

        framebuffers
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
    ) {
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

        let command_buffer = cmd_builder.build().unwrap(); // TODO: handle error

        // TODO: Unfinished
        todo!()
    }

    fn draw_frame() {
        // TODO: Stopped here 8:02 10.12.2024
    }
}

type CmdBuilder =
    AutoCommandBufferBuilder<PrimaryAutoCommandBuffer<StandardCommandBufferAllocator>>;
