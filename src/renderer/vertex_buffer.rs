use std::sync::Arc;

use vulkano::{
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
    pipeline::graphics::vertex_input::Vertex,
};

use super::Renderer;

#[derive(BufferContents, Vertex)]
#[repr(C)]
pub struct MyVertex {
    #[format(R32G32_SFLOAT)]
    pub pos: [f32; 2],
    #[format(R32G32B32A32_SFLOAT)]
    pub color: [f32; 4]
}

impl Renderer {
    pub(super) fn create_vertex_buffer(memory_allocator: Arc<StandardMemoryAllocator>, data: Vec<MyVertex>) -> Subbuffer<[MyVertex]> {

        let create_info = BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        };

        let allocation_info = AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        };

        Buffer::from_iter(
            memory_allocator,
            create_info,
            allocation_info,
            data,
        )
        .unwrap() // TODO: handle error
    }
}
