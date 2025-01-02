use std::sync::Arc;

use vulkano::{
    device::Device,
    format::Format,
    image::{ImageLayout, SampleCount},
    render_pass::{
        AttachmentDescription, AttachmentLoadOp, AttachmentReference, AttachmentStoreOp,
        RenderPass, RenderPassCreateInfo, SubpassDescription,
    }, single_pass_renderpass,
};

use super::Renderer;

impl Renderer {
    pub(super) fn create_render_pass(device: Arc<Device>, image_format: Format) -> Arc<RenderPass> {
        // let attachment_description = AttachmentDescription {
        //     format: image_format,
        //     samples: SampleCount::Sample1,
        //     load_op: AttachmentLoadOp::Clear,
        //     store_op: AttachmentStoreOp::Store,
        //     initial_layout: ImageLayout::Undefined,
        //     final_layout: ImageLayout::PresentSrc,
        //     ..Default::default()
        // };

        // let attachment_reference = AttachmentReference {
        //     attachment: 0,
        //     layout: ImageLayout::ColorAttachmentOptimal,
        //     ..Default::default()
        // };

        // let subpass_description = SubpassDescription {
        //     color_attachments: vec![Some(attachment_reference)],
        //     ..Default::default()
        // };

        // RenderPass::new(
        //     device.clone(),
        //     RenderPassCreateInfo {
        //         attachments: vec![attachment_description],
        //         subpasses: vec![subpass_description],
        //         ..Default::default()
        //     },
        // )
        // .unwrap() // TODO: handle error
        single_pass_renderpass!(
            device,
            attachments: {
                color: {
                    format: image_format,
                    samples: 1,
                    load_op: Clear,
                    store_op: Store,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            },
        ).unwrap()
    }
}
