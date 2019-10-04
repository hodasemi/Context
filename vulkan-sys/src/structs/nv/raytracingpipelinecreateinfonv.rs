use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkRayTracingPipelineCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCreateFlagBits,
    pub stageCount: u32,
    pub pStages: *const VkPipelineShaderStageCreateInfo,
    pub groupCount: u32,
    pub pGroups: *const VkRayTracingShaderGroupCreateInfoNV,
    pub maxRecursionDepth: u32,
    pub layout: VkPipelineLayout,
    pub basePipelineHandle: VkPipeline,
    pub basePipelineIndex: i32,
}

impl VkRayTracingPipelineCreateInfoNV {
    pub fn new<T>(
        flags: T,
        stages: &[VkPipelineShaderStageCreateInfo],
        groups: &[VkRayTracingShaderGroupCreateInfoNV],
        max_recursion_depth: u32,
        layout: VkPipelineLayout,
    ) -> Self
    where
        T: Into<VkPipelineCreateFlagBits>,
    {
        VkRayTracingPipelineCreateInfoNV {
            sType: VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_NV,
            pNext: ptr::null(),
            flags: flags.into(),
            stageCount: stages.len() as u32,
            pStages: stages.as_ptr(),
            groupCount: groups.len() as u32,
            pGroups: groups.as_ptr(),
            maxRecursionDepth: max_recursion_depth,
            layout,
            basePipelineHandle: VkPipeline::NULL_HANDLE,
            basePipelineIndex: -1,
        }
    }
}
