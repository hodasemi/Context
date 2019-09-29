use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkDescriptorPoolCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDescriptorPoolCreateFlagBits,
    pub maxSets: u32,
    pub poolSizeCount: u32,
    pub pPoolSizes: *const VkDescriptorPoolSize,
}

impl VkDescriptorPoolCreateInfo {
    pub fn new<'a, 'b: 'a, T>(
        flags: T,
        max_sets: u32,
        pool_sizes: &'b [VkDescriptorPoolSize],
    ) -> Self
    where
        T: Into<VkDescriptorPoolCreateFlagBits>,
    {
        VkDescriptorPoolCreateInfo {
            sType: VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            maxSets: max_sets,
            poolSizeCount: pool_sizes.len() as u32,
            pPoolSizes: pool_sizes.as_ptr(),
        }
    }
}
