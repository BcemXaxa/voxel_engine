mod for_multi;
mod vk_application;
use std::u64;

use ash::{
    self,
    vk::{self},
};
use gpu_allocator::{vulkan::*, MemoryLocation};

mod hardcoded_consts {
    pub const QUEUE_FAMILY_INDEX: u32 = 0;
    pub const QUEUE_INDEX: u32 = 0;
    pub const QUEUE_PRIORITIES: [f32; 1] = [1.0];
    pub const COMMAND_BUFFER_COUNT: u32 = 1;
}

fn main() {
    // Context
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
            .expect("Failed to create logical device.")
    };

    // Call vkEnumerateInstanceExtensionProperties
    let mut extension_names = Vec::new();
    match unsafe{entry.enumerate_instance_extension_properties(None)} {
        Ok(properties) => {
            for prop in properties {
                let name = prop.extension_name_as_c_str().unwrap().to_string_lossy().into_owned();
                extension_names.push(name);
            }
        },
        Err(e) => {
            println!("Failed to enumerate instance extension properties: {:?}", e);
        }
    }
    println!("Available Vulkan instance extensions:");
    for name in extension_names {
        println!("{:?}", name);
    }

    let queue = unsafe {
        device.get_device_queue(
            hardcoded_consts::QUEUE_FAMILY_INDEX,
            hardcoded_consts::QUEUE_INDEX,
        )
    };

    // Create allocator
    let mut allocator = {
        let allocator_desc = AllocatorCreateDesc {
            instance: instance.clone(),
            device: device.clone(),
            physical_device,
            buffer_device_address: false,
            debug_settings: Default::default(),
            allocation_sizes: Default::default(),
        };
        Allocator::new(&allocator_desc).expect("Failed to create allocator.")
    };

    let data: u32 = 10;
    let data_count = 64;

    // Create buffer
    let buffer = {
        let create_info = vk::BufferCreateInfo {
            size: (data_count * size_of::<u32>()) as vk::DeviceSize,
            usage: vk::BufferUsageFlags::TRANSFER_DST,
            ..vk::BufferCreateInfo::default()
        };
        unsafe { device.create_buffer(&create_info, None) }.expect("Failed to create buffer.")
    };

    let allocation = {
        let memory_requirements = unsafe { device.get_buffer_memory_requirements(buffer) };
        let allocation_description = AllocationCreateDesc {
            name: "Buffer allocation",
            requirements: memory_requirements,
            location: MemoryLocation::GpuToCpu,
            linear: true,
            allocation_scheme: AllocationScheme::DedicatedBuffer(buffer),
        };
        let allocation = allocator
            .allocate(&allocation_description)
            .expect("Failed to allocate buffer.");
        unsafe { device.bind_buffer_memory(buffer, allocation.memory(), allocation.offset()) }
            .expect("Failed to bind buffer memory.");
        allocation
    };

    let command_pool = {
        let create_info = vk::CommandPoolCreateInfo {
            queue_family_index: hardcoded_consts::QUEUE_FAMILY_INDEX,
            ..vk::CommandPoolCreateInfo::default()
        };
        unsafe { device.create_command_pool(&create_info, None) }
            .expect("Failed to create command pool.")
    };

    let command_buffer = {
        let allocate_info = vk::CommandBufferAllocateInfo {
            command_pool,
            command_buffer_count: hardcoded_consts::COMMAND_BUFFER_COUNT,
            level: vk::CommandBufferLevel::PRIMARY,
            ..vk::CommandBufferAllocateInfo::default()
        };
        unsafe { device.allocate_command_buffers(&allocate_info) }
            .unwrap()
            .into_iter()
            .next()
            .expect("Failed to create command buffer.")
    };

    // Recording command buffer
    {
        let begin_info = vk::CommandBufferBeginInfo::default();
        unsafe { device.begin_command_buffer(command_buffer, &begin_info) }
            .expect("Failed to begin command buffer.");
    }

    unsafe {
        device.cmd_fill_buffer(
            command_buffer,
            buffer,
            allocation.offset(),
            allocation.size(),
            data,
        )
    };

    {
        unsafe { device.end_command_buffer(command_buffer) }
            .expect("Failed to end command buffer.");
    }

    // Creating synchronization object (Fence)
    let fence = {
        let create_info = vk::FenceCreateInfo::default();
        unsafe { device.create_fence(&create_info, None) }.expect("Failed to create fence.")
    };

    // Execute commmand buffer by uploading it to the GPU through the queue
    {
        let submit_info =
            vk::SubmitInfo::default().command_buffers(std::slice::from_ref(&command_buffer));
        unsafe { device.queue_submit(queue, std::slice::from_ref(&submit_info), fence) }
            .expect("Failed to submit queue.")
    }

    // Wait for the execution to complete
    unsafe {
        device
            .wait_for_fences(std::slice::from_ref(&fence), true, u64::MAX)
            .unwrap()
    };

    // Read data
    let buffer_data: &[u32] = bytemuck::cast_slice(
        allocation
            .mapped_slice()
            .expect("Failed to access buffer from Host."),
    );

    // for value in buffer_data {
    //     println!("{value}");
    // }

    // Cleanup
    unsafe {
        device.destroy_fence(fence, None);
        device.destroy_command_pool(command_pool, None);
        allocator.free(allocation).unwrap();
        drop(allocator);
        device.destroy_buffer(buffer, None);
        device.destroy_device(None);
        instance.destroy_instance(None);
    }
}
