mod for_multi;
use ash::{
    self,
    vk::{self},
};

mod hardcoded_consts {
    pub const QUEUE_FAMILY_INDEX: u32 = 0;
    pub const QUEUE_INDEX: u32 = 0;
    pub const QUEUE_PRIORITIES: [f32; 1] = [1.0];
}

fn main() {
    let entry = unsafe { ash::Entry::load() }.expect(
        "Failed to create entry due to lack of Vulkan support. Consider installing Vulkan SDK.",
    );

    let instance = {
        let application_info = vk::ApplicationInfo {
            api_version: vk::API_VERSION_1_3,
            ..vk::ApplicationInfo::default()
        };
        let create_info = vk::InstanceCreateInfo {
            p_application_info: &application_info,
            ..vk::InstanceCreateInfo::default()
        };
        unsafe { entry.create_instance(&create_info, None) }.expect("Failed to create instance.")
    };

    let physical_device = {
        unsafe { instance.enumerate_physical_devices().unwrap() }
            .into_iter()
            .next()
            .expect("Failed to find physical device.")
    };

    let device = {
        let queue_create_infos = [vk::DeviceQueueCreateInfo::default()
            .queue_family_index(hardcoded_consts::QUEUE_FAMILY_INDEX)
            .queue_priorities(&hardcoded_consts::QUEUE_PRIORITIES)];
        let create_info = vk::DeviceCreateInfo::default().queue_create_infos(&queue_create_infos);
        unsafe { instance.create_device(physical_device, &create_info, None) }
            .expect("Failed to create logical device")
    };

    let queue = unsafe {
        device.get_device_queue(
            hardcoded_consts::QUEUE_FAMILY_INDEX,
            hardcoded_consts::QUEUE_INDEX,
        )
    };

    let command_pool = {
        let create_info = vk::CommandPoolCreateInfo {
            queue_family_index: hardcoded_consts::QUEUE_FAMILY_INDEX,
            ..vk::CommandPoolCreateInfo::default()
        };
        unsafe { device.create_command_pool(&create_info, None) }
            .expect("Failed to create command pool")
    };

    unsafe {
        device.destroy_command_pool(command_pool, None);
        device.destroy_device(None);
        instance.destroy_instance(None);
    }
}
