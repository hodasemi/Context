use utilities::prelude::*;

use crate::impl_vk_handle;
use crate::prelude::*;

use std::sync::Arc;

#[derive(Debug)]
pub struct PipelineLayout {
    device: Arc<Device>,
    pipeline_layout: VkPipelineLayout,
}

impl PipelineLayout {
    pub fn new(
        device: Arc<Device>,
        descriptor_set_layouts: &[&dyn VkHandle<VkDescriptorSetLayout>],
        push_constant_ranges: &[VkPushConstantRange],
    ) -> VerboseResult<Arc<PipelineLayout>> {
        let set_layouts: Vec<VkDescriptorSetLayout> = descriptor_set_layouts
            .iter()
            .map(|set_layout| set_layout.vk_handle())
            .collect();

        let pipeline_layout_ci = VkPipelineLayoutCreateInfo::new(
            VK_PIPELINE_LAYOUT_CREATE_NULL_BIT,
            set_layouts.as_slice(),
            push_constant_ranges,
        );

        let pipeline_layout = device.create_pipeline_layout(&pipeline_layout_ci)?;

        Ok(Arc::new(PipelineLayout {
            device,
            pipeline_layout,
        }))
    }
}

impl_vk_handle!(PipelineLayout, VkPipelineLayout, pipeline_layout);

impl Drop for PipelineLayout {
    fn drop(&mut self) {
        self.device.destroy_pipeline_layout(self.pipeline_layout);
    }
}
