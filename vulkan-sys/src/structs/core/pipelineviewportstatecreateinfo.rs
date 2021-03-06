use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkPipelineViewportStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineViewportStateCreateFlagBits,
    pub viewportCount: u32,
    pub pViewports: *const VkViewport,
    pub scissorCount: u32,
    pub pScissors: *const VkRect2D,
}

impl VkPipelineViewportStateCreateInfo {
    pub fn new<T>(
        flags: T,
        viewports: &[VkViewport],
        scissors: &[VkRect2D],
    ) -> VkPipelineViewportStateCreateInfo
    where
        T: Into<VkPipelineViewportStateCreateFlagBits>,
    {
        VkPipelineViewportStateCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            viewportCount: viewports.len() as u32,
            pViewports: viewports.as_ptr(),
            scissorCount: scissors.len() as u32,
            pScissors: scissors.as_ptr(),
        }
    }
}
