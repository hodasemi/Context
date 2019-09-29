use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkBufferViewCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkBufferViewCreateFlagBits,
    pub buffer: VkBuffer,
    pub format: VkFormat,
    pub offset: VkDeviceSize,
    pub range: VkDeviceSize,
}

impl VkBufferViewCreateInfo {
    pub fn new<T>(
        flags: T,
        buffer: VkBuffer,
        format: VkFormat,
        offset: VkDeviceSize,
        range: VkDeviceSize,
    ) -> Self
    where
        T: Into<VkBufferViewCreateFlagBits>,
    {
        VkBufferViewCreateInfo {
            sType: VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            buffer,
            format,
            offset,
            range,
        }
    }
}
