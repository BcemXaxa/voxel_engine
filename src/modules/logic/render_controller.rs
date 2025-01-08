use std::{cell::Cell, rc::Rc, sync::Arc};

use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer},
    command_buffer::{
        allocator::StandardCommandBufferAllocator, RenderPassBeginInfo, SubpassBeginInfo,
        SubpassEndInfo,
    },
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
    pipeline::{graphics::viewport::Viewport, GraphicsPipeline, Pipeline},
    render_pass::RenderPass,
};

use crate::modules::{
    math::{
        cg::*,
        mat::*,
    },
    renderer::{queue::QueueType, Renderer},
};

use super::{
    camera::Camera, chunk_mesher::{self, ChunkMeshVertex}, chunk_render::{self, ChunkPushConstant}, scene::Scene
};

pub struct RenderController {
    renderer: Renderer,

    scene: Rc<Scene>,
    frustum: PerspectiveFrustum,

    cmd_allocator: Arc<StandardCommandBufferAllocator>,
    mem_allocator: Arc<StandardMemoryAllocator>,

    render_pass: Arc<RenderPass>,
    chunk_pipeline: Arc<GraphicsPipeline>,

    chunk_vertices: Vec<Subbuffer<[ChunkMeshVertex]>>,
}

impl RenderController {
    pub fn new(renderer: Renderer, scene: Rc<Scene>) -> Self {
        let cmd_allocator = Arc::new(renderer.create_command_buffer_allocator());
        let mem_allocator = Arc::new(renderer.create_memory_allocator());

        let render_pass = renderer.default_render_pass(1);
        let chunk_pipeline = renderer.create_graphics_pipeline(|| {
            chunk_render::chunk_graphics_pipeline(&renderer, render_pass.clone().first_subpass())
        });

        let frustum = PerspectiveFrustum {
            near: 1e-1,
            far: 1e5,
            fov: 90.0,
            ar: renderer.swapchain_extent().aspect_ratio(),
        };

        let allocation_info = AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        };
        let create_info = BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        };
        let buffer = Buffer::from_iter(
            mem_allocator.clone(),
            create_info,
            allocation_info,
            chunk_mesher::mesh(scene.get_chunk([0, 0, 0]).unwrap()),
        )
        .unwrap();
        let mut chunk_vertices = Vec::new();
        chunk_vertices.push(buffer);

        Self {
            renderer,

            scene,
            frustum,

            cmd_allocator,
            mem_allocator,

            render_pass,
            chunk_pipeline,

            chunk_vertices,
        }
    }

    pub fn extent_changed(&mut self, extent: [u32; 2]) {
        self.renderer.recreate_swapchain(extent);
        self.frustum.ar = extent.aspect_ratio();
    }

    pub fn draw_frame(&self) {
        let (mut cmd_builder, _) = self
            .renderer
            .create_command_buffer_builder(QueueType::GraphicsPresent, &self.cmd_allocator);

        let viewports = {
            let extent = self.renderer.swapchain_extent();

            vec![Viewport {
                offset: [0.0; 2],
                extent: [extent[0] as f32, extent[1] as f32],
                depth_range: 0.0..=1.0,
            }]
        };
        let draw_result =
            self.renderer
                .execute_then_present(vec![self.render_pass.clone()], |framebuffers| {
                    cmd_builder
                        .set_viewport(0, viewports.into())
                        .unwrap()
                        .push_constants(
                            self.chunk_pipeline.layout().clone(),
                            0,
                            ChunkPushConstant {
                                pvm: self.frustum.projection_matrix().mult(self.scene.camera.borrow().view_matrix()).trans(),
                            },
                        )
                        .unwrap()
                        .begin_render_pass(
                            RenderPassBeginInfo {
                                clear_values: vec![Some([0.0, 0.0, 0.0, 1.0].into())],
                                ..RenderPassBeginInfo::framebuffer(framebuffers[0].clone())
                            },
                            SubpassBeginInfo::default(),
                        )
                        .unwrap()
                        .bind_pipeline_graphics(self.chunk_pipeline.clone())
                        .unwrap();
                    for subbuffer in &self.chunk_vertices {
                        cmd_builder
                            .bind_vertex_buffers(0, subbuffer.clone())
                            .unwrap()
                            .draw(subbuffer.len() as u32, 1, 0, 0)
                            .unwrap();
                    }
                    cmd_builder
                        .end_render_pass(SubpassEndInfo::default())
                        .unwrap();

                    cmd_builder.build().unwrap()
                });
        // match draw_result {
        //     Ok(_) => println!("everything is nice"),
        //     Err(_) => println!("damn that's crazy"),
        // }
    }

    pub fn fov_plus(&mut self) {
        self.frustum.fov += 1.0;
    }

    pub fn fov_minus(&mut self) {
        self.frustum.fov -= 1.0;
    }
}
