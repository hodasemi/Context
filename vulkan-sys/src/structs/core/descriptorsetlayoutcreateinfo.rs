use crate::impl_pnext;
use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkDescriptorSetLayoutCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDescriptorSetLayoutCreateFlagBits,
    pub bindingCount: u32,
    pub pBindings: *const VkDescriptorSetLayoutBinding,
}

impl VkDescriptorSetLayoutCreateInfo {
    pub fn new<T>(flags: T, bindings: &[VkDescriptorSetLayoutBinding]) -> Self
    where
        T: Into<VkDescriptorSetLayoutCreateFlagBits>,
    {
        VkDescriptorSetLayoutCreateInfo {
            sType: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            bindingCount: bindings.len() as u32,
            pBindings: bindings.as_ptr(),
        }
    }
}

impl_pnext!(
    VkDescriptorSetLayoutCreateInfo,
    VkDescriptorSetLayoutBindingFlagsCreateInfoEXT
);
