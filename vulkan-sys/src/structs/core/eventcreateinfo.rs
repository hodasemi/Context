use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkEventCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkEventCreateFlagBits,
}

impl VkEventCreateInfo {
    pub fn new<T>(flags: T) -> Self
    where
        T: Into<VkEventCreateFlagBits>,
    {
        VkEventCreateInfo {
            sType: VK_STRUCTURE_TYPE_EVENT_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
        }
    }
}
