use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkPipelineTessellationStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineTessellationStateCreateFlagBits,
    pub patchControlPoints: u32,
}

impl VkPipelineTessellationStateCreateInfo {
    pub fn new<T>(flags: T, patch_control_points: u32) -> VkPipelineTessellationStateCreateInfo
    where
        T: Into<VkPipelineTessellationStateCreateFlagBits>,
    {
        VkPipelineTessellationStateCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            patchControlPoints: patch_control_points,
        }
    }
}
