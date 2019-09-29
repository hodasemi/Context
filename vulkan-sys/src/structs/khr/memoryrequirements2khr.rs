use crate::prelude::*;

use std::os::raw::c_void;

#[repr(C)]
#[derive(Debug)]
pub struct VkMemoryRequirements2 {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memoryRequirements: VkMemoryRequirements,
}

pub type VkMemoryRequirements2KHR = VkMemoryRequirements2;
