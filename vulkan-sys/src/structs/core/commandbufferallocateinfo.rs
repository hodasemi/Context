use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkCommandBufferAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub commandPool: VkCommandPool,
    pub level: VkCommandBufferLevel,
    pub commandBufferCount: u32,
}

impl VkCommandBufferAllocateInfo {
    pub fn new(
        command_pool: VkCommandPool,
        level: VkCommandBufferLevel,
        command_buffer_count: u32,
    ) -> Self {
        VkCommandBufferAllocateInfo {
            sType: VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
            pNext: ptr::null(),
            commandPool: command_pool,
            level,
            commandBufferCount: command_buffer_count,
        }
    }
}
