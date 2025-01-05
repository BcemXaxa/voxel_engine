use vulkano::{command_buffer::allocator::StandardCommandBufferAllocator, memory::allocator::StandardMemoryAllocator};

use crate::modules::renderer::{queue::QueueType, Renderer};

use super::scene::Scene;

pub struct RenderController<'a> {
    renderer: &'a Renderer,
    scene: &'a Scene,

    cmd_allocator: StandardCommandBufferAllocator,
    mem_allocator: StandardMemoryAllocator,
}

impl<'a> RenderController<'a> {
    pub fn new(renderer: &'a Renderer, scene: &'a Scene) -> Self {

        let cmd_allocator = renderer.create_command_buffer_allocator();
        let mem_allocator = renderer.create_memory_allocator();

        Self{
            renderer,
            scene,
            cmd_allocator,
            mem_allocator,
        }
    }

    fn draw_frame(&self) {
        let (cmd_builder, _queue) = self
                    .renderer
                    .create_command_buffer_builder(QueueType::GraphicsPresent, cmd_allocator);
                
                self.renderer.execute_then_present(vec![], |framebuffers| {
                    let viewports = vec![Viewport {
                        offset: [0.0; 2],
                        extent: self.window.inner_size().into(),
                        depth_range: 0.0..=1.0,
                    }];
                    cmd_builder
                        .set_viewport(0, viewports.into())
                        .unwrap()
                        .begin_render_pass(
                            RenderPassBeginInfo::framebuffer(framebuffers[0]),
                            SubpassBeginInfo::default(),
                        )
                        .unwrap()
                        .bind_pipeline_graphics(pipeline)
                        .unwrap()
                        .bind_vertex_buffers(0, vertex_buffers)
                        .unwrap()
                    // .draw(, , 0, 0)
                    // .unwrap()
                    // .end_render_pass(SubpassEndInfo::default())
                    // .unwrap()

                    cmd_builder.build().unwrap()
                });
    }

}