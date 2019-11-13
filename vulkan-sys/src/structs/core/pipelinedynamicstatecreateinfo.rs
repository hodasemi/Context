use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkPipelineDynamicStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineDynamicStateCreateFlagBits,
    pub dynamicStateCount: u32,
    pub pDynamicStates: *const VkDynamicState,
}

impl VkPipelineDynamicStateCreateInfo {
    pub fn new<T>(flags: T, dynamic_states: &[VkDynamicState]) -> VkPipelineDynamicStateCreateInfo
    where
        T: Into<VkPipelineDynamicStateCreateFlagBits>,
    {
        VkPipelineDynamicStateCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            dynamicStateCount: dynamic_states.len() as u32,
            pDynamicStates: dynamic_states.as_ptr(),
        }
    }
}
