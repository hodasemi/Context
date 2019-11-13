use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkMappedMemoryRange {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memory: VkDeviceMemory,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
}

impl VkMappedMemoryRange {
    pub fn new(memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize) -> Self {
        VkMappedMemoryRange {
            sType: VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE,
            pNext: ptr::null(),
            memory,
            offset,
            size,
        }
    }
}
