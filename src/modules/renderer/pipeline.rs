use std::sync::Arc;

use vulkano::{
    device::Device,
    pipeline::{
        graphics::GraphicsPipelineCreateInfo, layout::PipelineLayoutCreateInfo, GraphicsPipeline,
        PipelineLayout,
    },
    shader::{EntryPoint, ShaderModule},
    Validated, VulkanError,
};

use super::Renderer;

impl Renderer {
    pub fn create_graphics_pipeline<F>(&self, graphics_pipeline: F) -> Arc<GraphicsPipeline>
    where
        F: FnOnce() -> GraphicsPipelineCreateInfo,
    {
        GraphicsPipeline::new(self.device.clone(), None, graphics_pipeline()).unwrap()
    }

    pub fn load_shader<F>(&self, shader: F) -> EntryPoint
    where
        F: FnOnce(Arc<Device>) -> Result<Arc<ShaderModule>, Validated<VulkanError>>,
    {
        shader(self.device.clone())
            .unwrap()
            .entry_point("main")
            .unwrap()
    }

    pub fn pipeline_layout(&self, create_info: PipelineLayoutCreateInfo) -> Arc<PipelineLayout> {
        PipelineLayout::new(self.device.clone(), create_info).unwrap()
    }
}
