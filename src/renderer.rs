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
        physical::{PhysicalDevice, PhysicalDeviceType},
        Device, DeviceCreateInfo, DeviceExtensions, Features, Queue, QueueCreateFlags,
        QueueCreateInfo, QueueFlags,
    },
    instance::{Instance, InstanceCreateInfo, InstanceExtensions},
    swapchain::Surface,
    Version, VulkanLibrary,
};
use winit::window::Window;

pub struct Renderer {
    pub library: Arc<VulkanLibrary>,
    pub instance: Arc<Instance>,
    pub physical_device: Arc<PhysicalDevice>,
    pub device: Arc<Device>,
    pub queues: Vec<Arc<Queue>>,
}

impl Renderer {
    pub fn run() {}

    pub fn new(
        required_extensions_receiver: Receiver<InstanceExtensions>,
        window_receiver: Receiver<Arc<Window>>,
    ) -> Self {
        let library = VulkanLibrary::new().expect("Library creation failed");

        let enabled_extensions = required_extensions_receiver.recv().unwrap();
        let instance = Self::new_instance(library.clone(), enabled_extensions);
        let surface = Surface::from_window(instance.clone(), window_receiver.recv().unwrap())
            .expect("Surface creation failed");

        let physical_device = Self::new_physical_device(instance.clone());
        let (device, queues) = Self::new_logical_device(physical_device.clone());

        Self {
            library,
            instance,
            physical_device,
            device,
            queues,
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

    fn new_physical_device(instance: Arc<Instance>) -> Arc<PhysicalDevice> {
        let physical_devices: Vec<_> = instance
            .enumerate_physical_devices()
            .expect("Physical devices enumeration failed")
            .filter(|physical_device| Self::is_physical_device_suitable(physical_device, &instance))
            .collect();

        physical_devices
            .into_iter()
            .next()
            .expect("No suitable physical devices found")
    }

    fn is_physical_device_suitable(physical_device: &PhysicalDevice, instance: &Instance) -> bool {
        let properties = physical_device.properties();
        let mut has_properties = true;
        has_properties &= properties.device_type == PhysicalDeviceType::DiscreteGpu;

        let features = physical_device.supported_features();
        let mut has_features = true;
        has_features &= features.geometry_shader;

        // TODO: make score system (optional)
        // TODO: make list of missing properties & features (optional)

        has_properties && has_features
    }

    fn new_logical_device(physical_device: Arc<PhysicalDevice>) -> (Arc<Device>, Vec<Arc<Queue>>) {
        let mut queue_families = physical_device.queue_family_properties();

        let graphics_queue_family = queue_families
            .iter()
            .enumerate()
            .find_map(|(index, family_properties)| {
                match family_properties.queue_flags.contains(QueueFlags::GRAPHICS) {
                    true => Some(index as u32),
                    false => None,
                }
            })
            .expect("Logical device creation failed: No graphics queues found");

        let transfer_queue_family = queue_families
            .iter()
            .enumerate()
            .find_map(|(index, family_properties)| {
                match family_properties.queue_flags.contains(QueueFlags::TRANSFER) {
                    true => Some(index as u32),
                    false => None,
                }
            })
            .expect("Logical device creation failed: No transfer queues found");

        let queue_indices = HashSet::from([graphics_queue_family, transfer_queue_family]);

        let mut queue_create_infos: Vec<QueueCreateInfo> = queue_indices
            .into_iter()
            .map(|queue_family_index| QueueCreateInfo {
                queue_family_index,
                queues: vec![1.0],
                ..Default::default()
            })
            .collect();

        let create_info = DeviceCreateInfo {
            queue_create_infos,
            enabled_extensions: DeviceExtensions::empty(),
            enabled_features: Features::empty(),
            ..Default::default()
        };

        let (device, queues) =
            Device::new(physical_device, create_info).expect("Logical device creation failed");
        (device, queues.collect())
    }
}
