use std::sync::Arc;

use vulkano::{
    buffer::Subbuffer, command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer, RenderPassBeginInfo, SubpassBeginInfo, SubpassContents, SubpassEndInfo
    }, device::Queue, pipeline::{graphics::viewport::Viewport, GraphicsPipeline}, render_pass::Framebuffer
};

use super::{vertex_buffer::MyVertex, Renderer};

type CmdBuilder =
    AutoCommandBufferBuilder<PrimaryAutoCommandBuffer<StandardCommandBufferAllocator>>;

impl Renderer {
    pub(super) fn recreate_command_buffers(&mut self) {
        self.command_buffers = Self::write_command_buffers(
            &self.command_buffer_allocator,
            self.queues.graphics_present().unwrap(),
            self.framebuffers.clone(),
            self.graphics_pipeline.clone(),
            self.vertex_buffer.clone(),
        )
    }

    pub(super) fn write_command_buffers(
        allocator: &StandardCommandBufferAllocator,
        queue: Arc<Queue>,
        framebuffers: Vec<Arc<Framebuffer>>,
        graphics_pipeline: Arc<GraphicsPipeline>,
        vertex_buffer: Subbuffer<[MyVertex]>
    ) -> Vec<(Arc<PrimaryAutoCommandBuffer>, Arc<Queue>)> {
        // TODO: handle errors

        framebuffers
            .into_iter()
            .map(|framebuffer| {
                let mut cmd_builder = Self::create_command_buffer_builder(allocator, queue.clone());
                cmd_builder
                    .begin_render_pass(
                        RenderPassBeginInfo {
                            clear_values: vec![Some([0.0, 0.0, 0.1, 1.0].into())], // TODO: make logic to handle possible framebuffer attachments
                            ..RenderPassBeginInfo::framebuffer(framebuffer)
                        },
                        SubpassBeginInfo{
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

                (cmd_builder.build().unwrap(), queue.clone()) // TODO: handle error
            })
            .collect()
    }

    fn create_command_buffer_builder(
        allocator: &StandardCommandBufferAllocator,
        queue: Arc<Queue>,
    ) -> CmdBuilder {
        AutoCommandBufferBuilder::primary(
            allocator,
            queue.queue_family_index(),
            CommandBufferUsage::MultipleSubmit,
        )
        .unwrap() // TODO: handle error
    }
}
