use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkBufferMemoryBarrier {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcAccessMask: VkAccessFlagBits,
    pub dstAccessMask: VkAccessFlagBits,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
}

impl VkBufferMemoryBarrier {
    pub fn new<S, T>(
        src_access_mask: S,
        dst_access_mask: T,
        src_queue_family_index: u32,
        dst_queue_family_index: u32,
        buffer: VkBuffer,
        offset: VkDeviceSize,
        size: VkDeviceSize,
    ) -> VkBufferMemoryBarrier
    where
        S: Into<VkAccessFlagBits>,
        T: Into<VkAccessFlagBits>,
    {
        VkBufferMemoryBarrier {
            sType: VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER,
            pNext: ptr::null(),
            srcAccessMask: src_access_mask.into(),
            dstAccessMask: dst_access_mask.into(),
            srcQueueFamilyIndex: src_queue_family_index,
            dstQueueFamilyIndex: dst_queue_family_index,
            buffer,
            offset,
            size,
        }
    }
}
