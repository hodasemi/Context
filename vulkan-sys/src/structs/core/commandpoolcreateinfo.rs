use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkCommandPoolCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkCommandPoolCreateFlagBits,
    pub queueFamilyIndex: u32,
}

impl VkCommandPoolCreateInfo {
    pub fn new<T>(flags: T, queue_family_index: u32) -> Self
    where
        T: Into<VkCommandPoolCreateFlagBits>,
    {
        VkCommandPoolCreateInfo {
            sType: VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            queueFamilyIndex: queue_family_index,
        }
    }
}
