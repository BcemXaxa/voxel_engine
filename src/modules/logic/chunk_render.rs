use std::collections::HashSet;

use vulkano::{
    buffer::BufferContents,
    pipeline::{
        graphics::{
            color_blend::{
                AttachmentBlend, BlendFactor, BlendOp, ColorBlendAttachmentState, ColorBlendState,
                ColorComponents,
            },
            input_assembly::{InputAssemblyState, PrimitiveTopology},
            multisample::MultisampleState,
            rasterization::{CullMode, FrontFace, PolygonMode, RasterizationState},
            vertex_input::{Vertex, VertexDefinition},
            viewport::{Viewport, ViewportState},
            GraphicsPipelineCreateInfo,
        },
        layout::{PipelineLayoutCreateInfo, PushConstantRange},
        DynamicState, PipelineShaderStageCreateInfo,
    },
    render_pass::{Subpass, SubpassDescription},
    shader::ShaderStages,
};

use crate::modules::{math::mat::Mat4x4, renderer::Renderer, shaders};

use super::chunk_mesher::ChunkMeshVertex;

pub fn chunk_subpass() -> SubpassDescription {
    Renderer::default_subpass()
}

pub fn chunk_graphics_pipeline(
    renderer: &Renderer,
    subpass: Subpass,
) -> GraphicsPipelineCreateInfo {
    let vertex_shader = renderer.load_shader(shaders::voxel_vertex_shader::load);
    let fragment_shader = renderer.load_shader(shaders::default_fragment_shader::load);

    let vertex_input_state = ChunkMeshVertex::per_vertex()
        .definition(&vertex_shader.info().input_interface)
        .unwrap();

    let pipeline_stages = vec![
        PipelineShaderStageCreateInfo::new(vertex_shader),
        PipelineShaderStageCreateInfo::new(fragment_shader),
    ];

    let rasterization_state = RasterizationState {
        polygon_mode: PolygonMode::Fill,
        cull_mode: CullMode::Back,
        front_face: FrontFace::CounterClockwise,
        ..Default::default()
    };

    let input_assembly_state = InputAssemblyState {
        topology: PrimitiveTopology::TriangleList,
        ..Default::default()
    };

    let color_blend_attachment_state = ColorBlendAttachmentState {
        blend: Some(AttachmentBlend {
            src_color_blend_factor: BlendFactor::One,
            dst_color_blend_factor: BlendFactor::Zero,
            color_blend_op: BlendOp::Add,
            src_alpha_blend_factor: BlendFactor::One,
            dst_alpha_blend_factor: BlendFactor::Zero,
            alpha_blend_op: BlendOp::Add,
        }),
        color_write_enable: true,
        color_write_mask: ColorComponents::all(),
    };

    let color_blend_state = ColorBlendState::with_attachment_states(
        subpass.num_color_attachments(),
        color_blend_attachment_state,
    );

    let layout = {
        let push_constant_ranges = vec![PushConstantRange {
            stages: ShaderStages::VERTEX,
            size: 16 * 4,
            ..Default::default()
        }];
        let create_info = PipelineLayoutCreateInfo {
            set_layouts: Default::default(),
            push_constant_ranges,
            ..Default::default()
        };
        renderer.pipeline_layout(create_info)
    };

    GraphicsPipelineCreateInfo {
        stages: pipeline_stages.into(),
        vertex_input_state: Some(vertex_input_state),
        input_assembly_state: Some(input_assembly_state),
        rasterization_state: Some(rasterization_state),
        viewport_state: Some(ViewportState::default()),
        multisample_state: Some(MultisampleState::default()),
        color_blend_state: Some(color_blend_state),
        subpass: Some(subpass.into()),
        dynamic_state: HashSet::from_iter(
            [DynamicState::Viewport].into_iter(),
        ),
        ..GraphicsPipelineCreateInfo::layout(layout)
    }
}

#[derive(BufferContents)]
#[repr(C)]
pub struct ChunkPushConstant {
    pub pvm: Mat4x4,
}
