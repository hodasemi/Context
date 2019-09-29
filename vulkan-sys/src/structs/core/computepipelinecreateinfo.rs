use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkComputePipelineCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCreateFlagBits,
    pub stage: VkPipelineShaderStageCreateInfo,
    pub layout: VkPipelineLayout,
    pub basePipelineHandle: VkPipeline,
    pub basePipelineIndex: i32,
}

impl VkComputePipelineCreateInfo {
    pub fn new<T>(
        flags: T,
        stage: VkPipelineShaderStageCreateInfo,
        layout: VkPipelineLayout,
    ) -> Self
    where
        T: Into<VkPipelineCreateFlagBits>,
    {
        VkComputePipelineCreateInfo {
            sType: VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            stage,
            layout,
            basePipelineHandle: VkPipeline::default(),
            basePipelineIndex: -1,
        }
    }

    pub fn set_base_pipeline(&mut self, pipeline: VkPipeline, index: i32) {
        self.basePipelineHandle = pipeline;
        self.basePipelineIndex = index;
    }
}
