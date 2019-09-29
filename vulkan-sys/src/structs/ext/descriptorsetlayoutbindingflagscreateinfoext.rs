use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkDescriptorSetLayoutBindingFlagsCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub bindingCount: u32,
    pub pBindingFlags: *const VkDescriptorBindingFlagBitsEXT,
}

impl VkDescriptorSetLayoutBindingFlagsCreateInfoEXT {
    pub fn new(binding_flags: &[VkDescriptorBindingFlagBitsEXT]) -> Self {
        VkDescriptorSetLayoutBindingFlagsCreateInfoEXT {
            sType: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO_EXT,
            pNext: ptr::null(),
            bindingCount: binding_flags.len() as u32,
            pBindingFlags: binding_flags.as_ptr(),
        }
    }
}
