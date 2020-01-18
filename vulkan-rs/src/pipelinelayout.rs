use utilities::prelude::*;

use crate::impl_vk_handle;
use crate::prelude::*;

use std::sync::Arc;

#[derive(Debug)]
pub struct PipelineLayoutBuilder {
    descriptor_set_layouts: Vec<VkDescriptorSetLayout>,
    push_constant_ranges: Vec<VkPushConstantRange>,
}

impl PipelineLayoutBuilder {
    pub fn add_descriptor_set_layout(
        mut self,
        descriptor_set_layout: &dyn VkHandle<VkDescriptorSetLayout>,
    ) -> Self {
        self.descriptor_set_layouts
            .push(descriptor_set_layout.vk_handle());

        self
    }

    pub fn add_push_constant(mut self, push_constant: VkPushConstantRange) -> Self {
        self.push_constant_ranges.push(push_constant);

        self
    }

    pub fn build(self, device: Arc<Device>) -> VerboseResult<Arc<PipelineLayout>> {
        let pipeline_layout_ci = VkPipelineLayoutCreateInfo::new(
            VK_PIPELINE_LAYOUT_CREATE_NULL_BIT,
            &self.descriptor_set_layouts,
            &self.push_constant_ranges,
        );

        let pipeline_layout = device.create_pipeline_layout(&pipeline_layout_ci)?;

        Ok(Arc::new(PipelineLayout {
            device,
            pipeline_layout,
        }))
    }
}

#[derive(Debug)]
pub struct PipelineLayout {
    device: Arc<Device>,
    pipeline_layout: VkPipelineLayout,
}

impl PipelineLayout {
    pub fn builder() -> PipelineLayoutBuilder {
        PipelineLayoutBuilder {
            descriptor_set_layouts: Vec::new(),
            push_constant_ranges: Vec::new(),
        }
    }
}

impl_vk_handle!(PipelineLayout, VkPipelineLayout, pipeline_layout);

impl Drop for PipelineLayout {
    fn drop(&mut self) {
        self.device.destroy_pipeline_layout(self.pipeline_layout);
    }
}
