use std::sync::Arc;

use vulkano::{
    format::Format,
    image::{view::ImageView, Image, ImageLayout, SampleCount},
    render_pass::{
        AttachmentDescription, AttachmentLoadOp, AttachmentReference, AttachmentStoreOp,
        RenderPass, RenderPassCreateInfo, SubpassDescription,
    },
};

use super::Renderer;

impl Renderer {
    pub fn create_render_pass(
        &self,
        attachments: Vec<AttachmentDescription>,
        subpasses: Vec<SubpassDescription>,
    ) -> Arc<RenderPass> {
        RenderPass::new(
            self.device.clone(),
            RenderPassCreateInfo {
                attachments,
                subpasses,
                ..Default::default()
            },
        )
        .unwrap() // TODO: handle error

        // single_pass_renderpass!(
        //     device,
        //     attachments: {
        //         color: {
        //             format: image_format,
        //             samples: 1,
        //             load_op: Clear,
        //             store_op: Store,
        //         }
        //     },
        //     pass: {
        //         color: [color],
        //         depth_stencil: {}
        //     },
        // ).unwrap()
    }

    pub fn default_render_pass(&self, subpass_count: usize) -> Arc<RenderPass> {
        let color_attachment = AttachmentDescription {
            format: self.swapchain.image_format(),
            samples: SampleCount::Sample1,
            load_op: AttachmentLoadOp::Clear,
            store_op: AttachmentStoreOp::Store,
            initial_layout: ImageLayout::Undefined, // TODO: check ColorAttachmentOptimal
            final_layout: ImageLayout::PresentSrc,
            ..Default::default()
        };

        self.create_render_pass(
            vec![color_attachment],
            vec![Self::default_subpass(); subpass_count],
        )
    }

    pub fn default_render_pass_with_depth(&self, subpass_count: usize) -> Arc<RenderPass> {
        let color_attachment = AttachmentDescription {
            format: self.swapchain.image_format(),
            samples: SampleCount::Sample1,
            load_op: AttachmentLoadOp::Clear,
            store_op: AttachmentStoreOp::Store,
            initial_layout: ImageLayout::Undefined, // TODO: check ColorAttachmentOptimal
            final_layout: ImageLayout::PresentSrc,
            ..Default::default()
        };
        let depth_attachment = AttachmentDescription {
            format: Format::D32_SFLOAT,
            samples: SampleCount::Sample1,
            load_op: AttachmentLoadOp::Clear,
            store_op: AttachmentStoreOp::DontCare,
            initial_layout: ImageLayout::Undefined,
            final_layout: ImageLayout::DepthStencilAttachmentOptimal,
            stencil_load_op: Some(AttachmentLoadOp::DontCare),
            stencil_store_op: Some(AttachmentStoreOp::DontCare),
            stencil_initial_layout: None,
            stencil_final_layout: None,
            ..Default::default()
        };
        let subpass = {
            let depth_ref = AttachmentReference {
                attachment: 1,
                layout: ImageLayout::DepthStencilAttachmentOptimal,
                ..Default::default()
            };
            SubpassDescription {
                depth_stencil_attachment: Some(depth_ref),
                ..Self::default_subpass()
            }
        };
        self.create_render_pass(
            vec![color_attachment, depth_attachment],
            vec![subpass; subpass_count],
        )
    }

    pub fn default_subpass() -> SubpassDescription {
        let color_ref = AttachmentReference {
            attachment: 0,
            layout: ImageLayout::ColorAttachmentOptimal,
            ..Default::default()
        };

        SubpassDescription {
            color_attachments: vec![Some(color_ref)],
            ..Default::default()
        }
    }
}
