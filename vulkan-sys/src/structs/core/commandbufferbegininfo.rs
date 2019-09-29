use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkCommandBufferBeginInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkCommandBufferUsageFlagBits,
    pub pInheritanceInfo: *const VkCommandBufferInheritanceInfo,
}

impl VkCommandBufferBeginInfo {
    pub fn new<T>(flags: T) -> Self
    where
        T: Into<VkCommandBufferUsageFlagBits>,
    {
        VkCommandBufferBeginInfo {
            sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            pInheritanceInfo: ptr::null(),
        }
    }

    pub fn set_inheritance_info<'a, 'b: 'a>(
        &'a mut self,
        inheritance_info: &'b VkCommandBufferInheritanceInfo,
    ) {
        self.pInheritanceInfo = inheritance_info as *const VkCommandBufferInheritanceInfo;
    }
}
