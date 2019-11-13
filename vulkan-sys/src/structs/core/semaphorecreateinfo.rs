use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkSemaphoreCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkSemaphoreCreateFlagBits,
}

impl VkSemaphoreCreateInfo {
    pub fn new<T>(flags: T) -> Self
    where
        T: Into<VkSemaphoreCreateFlagBits>,
    {
        VkSemaphoreCreateInfo {
            sType: VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
        }
    }
}
