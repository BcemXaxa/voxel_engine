use std::sync::Arc;

use vulkano::{
    device::{
        physical::PhysicalDevice, Device, DeviceCreateInfo, DeviceExtensions, Features,
        QueueCreateInfo, QueueFlags,
    },
    swapchain::Surface,
};

use super::{queue::Queues, Renderer};

impl Renderer {
    pub(super) fn create_logical_device(
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
}
