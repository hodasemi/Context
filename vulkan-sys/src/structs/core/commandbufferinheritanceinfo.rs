use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkCommandBufferInheritanceInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub renderPass: VkRenderPass,
    pub subpass: u32,
    pub framebuffer: VkFramebuffer,
    pub occlusionQueryEnable: VkBool32,
    pub queryFlagBits: VkQueryControlFlagBits,
    pub pipelineStatistics: VkQueryPipelineStatisticFlagBits,
}

impl VkCommandBufferInheritanceInfo {
    pub fn new(renderpass: VkRenderPass, subpass: u32, framebuffer: VkFramebuffer) -> Self {
        VkCommandBufferInheritanceInfo {
            sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO,
            pNext: ptr::null(),
            renderPass: renderpass,
            subpass,
            framebuffer,
            occlusionQueryEnable: VK_FALSE,
            queryFlagBits: 0u32.into(),
            pipelineStatistics: 0u32.into(),
        }
    }

    pub fn set_query<T, U>(
        &mut self,
        occlusion_query_enable: bool,
        query_flag: T,
        pipeline_statisctics: U,
    ) where
        T: Into<VkQueryControlFlagBits>,
        U: Into<VkQueryPipelineStatisticFlagBits>,
    {
        self.occlusionQueryEnable = occlusion_query_enable.into();
        self.queryFlagBits = query_flag.into();
        self.pipelineStatistics = pipeline_statisctics.into();
    }
}
