use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkDescriptorSetVariableDescriptorCountAllocateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub descriptorSetCount: u32,
    pub pDescriptorCounts: *const u32,
}

impl VkDescriptorSetVariableDescriptorCountAllocateInfoEXT {
    pub fn new(descriptor_counts: &[u32]) -> Self {
        VkDescriptorSetVariableDescriptorCountAllocateInfoEXT {
            sType: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO_EXT,
            pNext: ptr::null(),
            descriptorSetCount: descriptor_counts.len() as u32,
            pDescriptorCounts: descriptor_counts.as_ptr(),
        }
    }

    pub fn set_descriptor_counts(&mut self, descriptor_counts: &[u32]) {
        self.descriptorSetCount = descriptor_counts.len() as u32;
        self.pDescriptorCounts = descriptor_counts.as_ptr();
    }
}
