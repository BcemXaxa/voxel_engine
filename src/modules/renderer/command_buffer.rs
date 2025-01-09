use std::sync::Arc;

use vulkano::{
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage,
        PrimaryAutoCommandBuffer,
    },
    device::Queue,
};

use super::{queue::QueueType, Renderer};

impl Renderer {
    pub fn create_command_buffer_builder(
        &self,
        queue_type: QueueType,
        allocator: &StandardCommandBufferAllocator,
    ) -> (CmdBuilder, Arc<Queue>) {
        let queue = self.queues.get(queue_type).unwrap();
        let cmd_bulider = AutoCommandBufferBuilder::primary(
            allocator,
            queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap(); // TODO: handle error
        (cmd_bulider, queue)
    }
}
type CmdBuilder =
    AutoCommandBufferBuilder<PrimaryAutoCommandBuffer<StandardCommandBufferAllocator>>;
