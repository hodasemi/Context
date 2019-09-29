use crate::impl_pnext;
use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

pub struct VkDescriptorSetLayoutSupport {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub supported: VkBool32,
}

impl VkDescriptorSetLayoutSupport {
    pub fn new(supported: impl Into<VkBool32>) -> Self {
        VkDescriptorSetLayoutSupport {
            sType: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT,
            pNext: ptr::null(),
            supported: supported.into(),
        }
    }
}

impl Default for VkDescriptorSetLayoutSupport {
    fn default() -> Self {
        Self::new(VK_FALSE)
    }
}

impl_pnext!(
    VkDescriptorSetLayoutSupport,
    VkDescriptorSetVariableDescriptorCountLayoutSupportEXT
);
