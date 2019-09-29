use crate::impl_pnext;
use crate::prelude::*;

use std::marker::PhantomData;
use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkDescriptorSetAllocateInfo<'a> {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub descriptorPool: VkDescriptorPool,
    pub descriptorSetCount: u32,
    layouts_lt: PhantomData<&'a VkDescriptorSetLayout>,
    pub pSetLayouts: *const VkDescriptorSetLayout,
}

impl<'a> VkDescriptorSetAllocateInfo<'a> {
    pub fn new(
        descriptor_pool: VkDescriptorPool,
        set_layouts: &'a [VkDescriptorSetLayout],
    ) -> Self {
        VkDescriptorSetAllocateInfo {
            sType: VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
            pNext: ptr::null(),
            descriptorPool: descriptor_pool,
            descriptorSetCount: set_layouts.len() as u32,
            layouts_lt: PhantomData,
            pSetLayouts: set_layouts.as_ptr(),
        }
    }
}

impl_pnext!(
    VkDescriptorSetAllocateInfo<'_>,
    VkDescriptorSetVariableDescriptorCountAllocateInfoEXT
);
