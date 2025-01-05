use std::sync::Arc;

use vulkano::{
    image::{ImageLayout, SampleCount},
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
        let attachment_description = AttachmentDescription {
            format: self.swapchain.image_format(),
            samples: SampleCount::Sample1,
            load_op: AttachmentLoadOp::Clear,
            store_op: AttachmentStoreOp::Store,
            initial_layout: ImageLayout::Undefined,
            final_layout: ImageLayout::PresentSrc,
            ..Default::default()
        };
        let subpasses = vec![Self::default_subpass(); subpass_count];

        self.create_render_pass(vec![attachment_description], subpasses)
    }

    pub fn default_subpass() -> SubpassDescription {
        let attachment_reference = AttachmentReference {
            attachment: 0,
            layout: ImageLayout::ColorAttachmentOptimal,
            ..Default::default()
        };

        SubpassDescription {
            color_attachments: vec![Some(attachment_reference)],
            ..Default::default()
        }
    }
}
