use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkImageMemoryBarrier {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcAccessMask: VkAccessFlagBits,
    pub dstAccessMask: VkAccessFlagBits,
    pub oldLayout: VkImageLayout,
    pub newLayout: VkImageLayout,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub image: VkImage,
    pub subresourceRange: VkImageSubresourceRange,
}

impl VkImageMemoryBarrier {
    pub fn new<S, T>(
        src_access_mask: S,
        dst_access_mask: T,
        old_layout: VkImageLayout,
        new_layout: VkImageLayout,
        src_queue_family_index: u32,
        dst_queue_family_index: u32,
        image: VkImage,
        subresource_range: VkImageSubresourceRange,
    ) -> VkImageMemoryBarrier
    where
        S: Into<VkAccessFlagBits>,
        T: Into<VkAccessFlagBits>,
    {
        VkImageMemoryBarrier {
            sType: VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
            pNext: ptr::null(),
            srcAccessMask: src_access_mask.into(),
            dstAccessMask: dst_access_mask.into(),
            oldLayout: old_layout,
            newLayout: new_layout,
            srcQueueFamilyIndex: src_queue_family_index,
            dstQueueFamilyIndex: dst_queue_family_index,
            image,
            subresourceRange: subresource_range,
        }
    }
}
