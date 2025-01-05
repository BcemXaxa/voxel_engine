use std::sync::Arc;

use vulkano::{
    buffer::Subbuffer,
    command_buffer::{
        allocator::{CommandBufferAllocator, StandardCommandBufferAllocator},
        AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer,
        RenderPassBeginInfo, SubpassBeginInfo, SubpassContents, SubpassEndInfo,
    },
    device::Queue,
    pipeline::GraphicsPipeline,
    render_pass::Framebuffer,
};

use super::{queue::QueueType, vertex_buffer::MyVertex, Renderer};

impl Renderer {
    fn write_command_buffer(
    ) {
        // TODO: handle errors

        cmd_builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![Some([0.0, 0.0, 0.1, 1.0].into())], // TODO: make logic to handle possible framebuffer attachments
                    ..RenderPassBeginInfo::framebuffer(framebuffer)
                },
                SubpassBeginInfo {
                    contents: SubpassContents::Inline,
                    ..Default::default()
                },
            )
            .unwrap()
            .bind_pipeline_graphics(graphics_pipeline.clone())
            .unwrap()
            .bind_vertex_buffers(0, vertex_buffer.clone())
            .unwrap()
            // .set_viewport(0, )
            // .unwrap()
            // .set_scissor(0, Default::default())
            // .unwrap()
            .draw(vertex_buffer.len() as u32, 1, 0, 0) // FIXME: hardcoded
            .unwrap()
            .end_render_pass(SubpassEndInfo::default())
            .unwrap();
    }

    pub fn create_command_buffer_builder(
        &self,
        queue_type: QueueType,
        allocator: StandardCommandBufferAllocator,
    ) -> (CmdBuilder, Arc<Queue>) {
        let queue = self.queues.get(queue_type).unwrap();
        let cmd_bulider = AutoCommandBufferBuilder::primary(
            &allocator,
            queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap(); // TODO: handle error
        (cmd_bulider, queue)
    }
}
type CmdBuilder =
    AutoCommandBufferBuilder<PrimaryAutoCommandBuffer<StandardCommandBufferAllocator>>;
