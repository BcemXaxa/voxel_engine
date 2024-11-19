#![allow(unused)]
use std::{
    collections::HashSet,
    default,
    sync::{mpsc::Receiver, Arc},
};

use gpu_allocator::vulkan::Allocator;
use vulkano::{
    self,
    device::{
        self,
        physical::{PhysicalDevice, PhysicalDeviceType},
        Device, DeviceCreateInfo, DeviceExtensions, Features, Queue, QueueCreateFlags,
        QueueCreateInfo, QueueFlags,
    },
    format::Format,
    image::{sampler::ComponentMapping, view::{ImageView, ImageViewCreateInfo, ImageViewType}, Image, ImageAspects, ImageSubresourceRange, ImageUsage},
    instance::{Instance, InstanceCreateInfo, InstanceExtensions},
    swapchain::{
        self, ColorSpace, PresentMode, Surface, SurfaceCapabilities, SurfaceInfo, Swapchain,
        SwapchainCreateInfo,
    },
    sync::Sharing,
    Version, VulkanLibrary, VulkanObject,
};
use winit::window::Window;

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
        
        let image_views = images.iter().map(|image| Self::get_image_view(image.clone())).collect::<Vec<_>>();

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
        let mut queue_families = physical_device.queue_family_properties();

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

        let mut queue_create_infos: Vec<QueueCreateInfo> = queue_indices
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

    fn get_image_view(image: Arc<Image>) -> Arc<ImageView>{
        let create_info = ImageViewCreateInfo{
            view_type: ImageViewType::Dim2d,
            format: image.format(),
            usage: image.usage(),
            component_mapping: ComponentMapping::identity(),
            subresource_range: ImageSubresourceRange::from_parameters(Format::B8G8R8A8_SRGB, image.mip_levels(), image.array_layers()),
            ..Default::default()
        };

        // TODO: handle error
        ImageView::new(image, create_info).unwrap()
    }
    
    fn create_graphics_pipeline() {
        let vertex_shader = // TODO: Finished here 20.11.2024 0:13
        todo!();
    }
}
