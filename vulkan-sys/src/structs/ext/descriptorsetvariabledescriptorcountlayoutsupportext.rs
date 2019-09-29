use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkDescriptorSetVariableDescriptorCountLayoutSupportEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub maxVariableDescriptorCount: u32,
}

impl VkDescriptorSetVariableDescriptorCountLayoutSupportEXT {
    pub fn new(max_variable_descriptor_count: u32) -> Self {
        VkDescriptorSetVariableDescriptorCountLayoutSupportEXT {
            sType: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT_EXT,
            pNext: ptr::null(),
            maxVariableDescriptorCount: max_variable_descriptor_count,
        }
    }
}

impl Default for VkDescriptorSetVariableDescriptorCountLayoutSupportEXT {
    fn default() -> Self {
        VkDescriptorSetVariableDescriptorCountLayoutSupportEXT::new(0)
    }
}
