use std::sync::Arc;

use vulkano::{
    device::Device,
    pipeline::{
        graphics::{
            color_blend::{
                AttachmentBlend, BlendFactor, BlendOp, ColorBlendAttachmentState, ColorBlendState,
                ColorComponents,
            },
            input_assembly::{InputAssemblyState, PrimitiveTopology},
            multisample::MultisampleState,
            rasterization::{
                CullMode, FrontFace, LineRasterizationMode, PolygonMode, RasterizationState,
            },
            subpass::PipelineSubpassType,
            vertex_input::VertexInputState,
            viewport::{Scissor, Viewport, ViewportState},
            GraphicsPipelineCreateInfo,
        },
        layout::PipelineLayoutCreateInfo,
        DynamicState, GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo,
    },
    render_pass::RenderPass,
};

use crate::shaders;

use super::Renderer;

impl Renderer {
    pub(super) fn recreate_graphics_pipeline(&mut self, extent: [u32; 2]) {
        // TODO: optimize?
        self.graphics_pipeline =
            Self::create_graphics_pipeline(self.device.clone(), self.render_pass.clone(), extent);
    }

    pub(super) fn create_graphics_pipeline(
        device: Arc<Device>,
        render_pass: Arc<RenderPass>,
        extent: [u32; 2],
    ) -> Arc<GraphicsPipeline> {
        // TODO: handle error
        let vertex_shader = shaders::default_vertex_shader::load(device.clone()).unwrap();
        // TODO: handle error
        let fragment_shader = shaders::default_fragment_shader::load(device.clone()).unwrap();

        let pipeline_stages = [
            {
                // TODO: handle error
                let entry_point = vertex_shader.entry_point("main").unwrap();
                PipelineShaderStageCreateInfo::new(entry_point)
            },
            {
                // TODO: handle error
                let entry_point = fragment_shader.entry_point("main").unwrap();
                PipelineShaderStageCreateInfo::new(entry_point)
            },
        ];

        let dynamic_states = [DynamicState::Viewport, DynamicState::Scissor];

        // TODO: real vertex input state
        let vertex_input_state = VertexInputState {
            ..Default::default()
        };

        let input_assembly_state = InputAssemblyState {
            topology: PrimitiveTopology::TriangleList,
            primitive_restart_enable: false,
            ..Default::default()
        };

        let viewport_state = {
            let viewport = Viewport {
                // FIXME: hardcoded?
                offset: [0.0, 0.0],
                extent: [extent[0] as f32, extent[1] as f32],
                depth_range: (0.0)..=(1.0),
            };

            let scissor = Scissor {
                // FIXME: hardcoded?
                offset: [0, 0],
                extent,
            };

            ViewportState {
                viewports: [viewport].into(),
                //scissors: [scissor].into(),
                ..Default::default()
            }
        };

        let rasterization_state = RasterizationState {
            depth_clamp_enable: false,
            rasterizer_discard_enable: false, // TODO: maybe change later
            polygon_mode: PolygonMode::Fill,
            cull_mode: CullMode::Back, // TODO: check if it is faster than other methods of backculling
            front_face: FrontFace::CounterClockwise,
            depth_bias: None,
            line_width: 1.0,
            line_rasterization_mode: LineRasterizationMode::Default,
            line_stipple: None,
            ..Default::default()
        };

        let multisample_state = MultisampleState {
            // TODO: use multisampling
            ..Default::default()
        };

        // TODO: maybe depth and stencil testing

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

        let color_blend_state = ColorBlendState {
            attachments: vec![color_blend_attachment_state],
            logic_op: None,
            blend_constants: [0.0; 4],
            flags: Default::default(),
            ..Default::default()
        };

        // TODO: handle error
        let pipeline_layout =
            PipelineLayout::new(device.clone(), PipelineLayoutCreateInfo::default()).unwrap();

        {
            let mut create_info = GraphicsPipelineCreateInfo::layout(pipeline_layout);
            create_info.stages.extend(pipeline_stages);
            create_info.vertex_input_state = Some(vertex_input_state);
            create_info.input_assembly_state = Some(input_assembly_state);
            create_info.viewport_state = Some(viewport_state);
            create_info.rasterization_state = Some(rasterization_state);
            create_info.multisample_state = Some(multisample_state);
            create_info.color_blend_state = Some(color_blend_state);
            //create_info.dynamic_state.extend(dynamic_states);
            create_info.subpass = Some(PipelineSubpassType::BeginRenderPass(
                render_pass.first_subpass(),
            ));
            create_info.base_pipeline = None;
            GraphicsPipeline::new(device.clone(), None, create_info)
        }
        .unwrap() // TODO: handle error
    }
}
