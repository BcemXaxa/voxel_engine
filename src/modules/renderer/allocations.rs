use std::sync::Arc;

use vulkano::{
    command_buffer::allocator::{
        StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
    },
    format::Format,
    image::{
        view::{ImageView, ImageViewCreateInfo},
        Image, ImageCreateInfo, ImageLayout, ImageType, ImageUsage,
    },
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
};

use super::Renderer;

impl Renderer {
    pub fn create_command_buffer_allocator(&self) -> StandardCommandBufferAllocator {
        StandardCommandBufferAllocator::new(
            self.device.clone(),
            StandardCommandBufferAllocatorCreateInfo::default(),
        )
    }

    pub fn create_memory_allocator(&self) -> StandardMemoryAllocator {
        StandardMemoryAllocator::new_default(self.device.clone())
    }

    pub fn create_depth_buffer(&self, allocator: Arc<StandardMemoryAllocator>) -> Arc<ImageView> {
        let extent = self.swapchain_extent();
        let create_info = ImageCreateInfo {
            image_type: ImageType::Dim2d,
            format: Format::D32_SFLOAT,
            extent: [extent[0], extent[1], 1],
            usage: ImageUsage::DEPTH_STENCIL_ATTACHMENT,
            initial_layout: ImageLayout::Undefined,
            ..Default::default()
        };
        let allocation_info = AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
            ..Default::default()
        };
        let depth_buffer = Image::new(allocator, create_info, allocation_info).unwrap();
        let create_info = ImageViewCreateInfo::from_image(&depth_buffer);
        ImageView::new(depth_buffer, create_info).unwrap()
    }
}
