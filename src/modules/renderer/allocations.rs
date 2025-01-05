use vulkano::{
    command_buffer::allocator::{
        StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
    },
    memory::allocator::StandardMemoryAllocator,
};

use super::Renderer;

impl Renderer {
    fn create_command_buffer_allocator(&self) -> StandardCommandBufferAllocator {
        StandardCommandBufferAllocator::new(
            self.device.clone(),
            StandardCommandBufferAllocatorCreateInfo::default(),
        )
    }

    fn create_memory_allocator(&self) -> StandardMemoryAllocator {
        StandardMemoryAllocator::new_default(self.device.clone())
    }
}
