use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkFenceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkFenceCreateFlagBits,
}

impl VkFenceCreateInfo {
    pub fn new<T>(flags: T) -> Self
    where
        T: Into<VkFenceCreateFlagBits>,
    {
        VkFenceCreateInfo {
            sType: VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
        }
    }
}
