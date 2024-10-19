#![allow(unused)]
use ash::{
    self,
    vk::{self, PhysicalDevice, QueueFamilyProperties},
    vk_bitflags_wrapped,
};
use std::collections::BinaryHeap;

pub struct VulkanApplication {
    pub entry: ash::Entry,
    pub instance: ash::Instance,
    pub physical_device: vk::PhysicalDevice,
    pub device: ash::Device,
}

impl Drop for VulkanApplication {
    // clean up
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}

impl VulkanApplication {
    pub fn run() {}

    fn new() -> Self {
        let entry = Self::new_entry();
        let instance = Self::new_instance(&entry);
        let physical_device = Self::new_physical_device(&instance);
        let device = Self::new_logical_device(physical_device, &instance);

        Self {
            entry,
            instance,
            physical_device,
            device,
        }
    }

    fn new_entry() -> ash::Entry {
        match unsafe { ash::Entry::load() } {
            Ok(entry) => entry,
            Err(error) => panic!("Entry creation failed: {error}"),
        }
    }

    fn new_instance(entry: &ash::Entry) -> ash::Instance {
        use std::ffi::CString;

        let application_name = CString::new("VoxelEngine").unwrap();
        let engine_name = CString::new("voxen").unwrap();
        let application_version = vk::make_api_version(
            0,
            env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
            env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
        );
        let engine_version = application_version;

        let application_info = vk::ApplicationInfo {
            p_application_name: application_name.as_ptr(),
            p_engine_name: engine_name.as_ptr(),
            application_version,
            engine_version,
            api_version: vk::API_VERSION_1_3,
            ..vk::ApplicationInfo::default()
        };

        let create_info = vk::InstanceCreateInfo {
            p_application_info: &application_info,
            ..vk::InstanceCreateInfo::default()
        };

        match unsafe { entry.create_instance(&create_info, None) } {
            Ok(instance) => instance,
            Err(error) => panic!("Instance creation failed: {error}"),
        }
    }

    fn new_physical_device(instance: &ash::Instance) -> vk::PhysicalDevice {
        let physical_devices: Vec<PhysicalDevice> =
            unsafe { instance.enumerate_physical_devices().unwrap() }
                .into_iter()
                .filter(|physical_device| {
                    Self::is_physical_device_suitable(physical_device, instance)
                })
                .collect();

        match physical_devices.into_iter().next() {
            Some(physical_device) => physical_device,
            None => panic!("No suitable physical devices found"),
        }
    }

    fn is_physical_device_suitable(
        physical_device: &vk::PhysicalDevice,
        instance: &ash::Instance,
    ) -> bool {
        let physical_device = physical_device.to_owned();

        let properties = unsafe { instance.get_physical_device_properties(physical_device) };
        let mut has_properties = true;
        has_properties &= properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU;

        let features = unsafe { instance.get_physical_device_features(physical_device) };
        let mut has_features = true;
        has_features &= features.geometry_shader.is();

        // TODO: make score system (optional)
        // TODO: make list of missing properties & features (optional)

        has_properties && has_features
    }

    fn new_logical_device(
        physical_device: vk::PhysicalDevice,
        instance: &ash::Instance,
    ) -> ash::Device {
        let mut queue_families = Self::find_queue_families(physical_device, instance);
        let queue_family_index = match queue_families.graphics.pop() {
            Some(priority_index) => priority_index.index(),
            None => panic!("Logical device creation failed: No graphics queues found"),
        };
        
        // TODO: stopped here, 19.10.24 3:14

        let queue_create_info = vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family_index)
            .queue_priorities(&[1.0]);
        let create_info = vk::DeviceCreateInfo::default();

        match unsafe { instance.create_device(physical_device, &create_info, None) } {
            Ok(device) => device,
            Err(error) => panic!("Logical device creation failed: {error}"),
        }
    }

    fn find_queue_families(
        physical_device: vk::PhysicalDevice,
        instance: &ash::Instance,
    ) -> QueueFamilies {
        let families_properties =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        let mut graphics = BinaryHeap::new();
        let mut compute = BinaryHeap::new();
        let mut transfer = BinaryHeap::new();

        const HIGH_PRIORITY: u8 = 1;
        const LOW_PRIORITY: u8 = 0;

        for (index, properties) in families_properties.into_iter().enumerate() {
            let index = index as u32;
            let mut priority = HIGH_PRIORITY;
            if properties.queue_flags.intersects(vk::QueueFlags::GRAPHICS) {
                graphics.push(QueueFamily {
                    priority,
                    index,
                    properties,
                });
                priority = LOW_PRIORITY;
            }
            if properties.queue_flags.intersects(vk::QueueFlags::COMPUTE) {
                compute.push(QueueFamily {
                    priority,
                    index,
                    properties,
                });
                priority = LOW_PRIORITY;
            }
            if properties.queue_flags.intersects(vk::QueueFlags::TRANSFER) {
                transfer.push(QueueFamily {
                    priority,
                    index,
                    properties,
                });
            }
        }

        QueueFamilies {
            graphics,
            compute,
            transfer,
        }
    }
}

struct QueueFamilies {
    pub graphics: BinaryHeap<QueueFamily>,
    pub compute: BinaryHeap<QueueFamily>,
    pub transfer: BinaryHeap<QueueFamily>,
}
struct QueueFamily {
    priority: u8,
    index: u32,
    properties: QueueFamilyProperties,
}
impl QueueFamily {
    fn index(&self) -> u32 {
        self.index
    }
}
impl PartialEq for QueueFamily {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.index == other.index
    }
}
impl Eq for QueueFamily {}
impl Ord for QueueFamily {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.priority.cmp(&other.priority) {
            std::cmp::Ordering::Equal => other.index.cmp(&self.index),
            cmp => cmp,
        }
    }
}
impl PartialOrd for QueueFamily {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

trait IntoBool {
    fn is(self) -> bool;
}
impl IntoBool for u32 {
    fn is(self) -> bool {
        self != 0
    }
}
