
use crate::prelude::*;
use crate::SetupU64Conv;

use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkDescriptorSet(u64);
SetupU64Conv!(VkDescriptorSet);

/*
impl VkDescriptorSet {
    pub fn allocate(
        device: VkDevice,
        descriptor_set_allocate_info: &VkDescriptorSetAllocateInfo,
    ) -> Result<Vec<VkDescriptorSet>, VkResult> {
        unsafe {
            let count = descriptor_set_allocate_info.descriptorSetCount as usize;

            let mut descriptor_sets = Vec::with_capacity(count);
            descriptor_sets.set_len(count);

            let result = vkAllocateDescriptorSets(
                device,
                descriptor_set_allocate_info,
                descriptor_sets.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(descriptor_sets)
            } else {
                Err(result)
            }
        }
    }

    pub fn update(
        device: VkDevice,
        writes: &[VkWriteDescriptorSet],
        copies: &[VkCopyDescriptorSet],
    ) {
        unsafe {
            vkUpdateDescriptorSets(
                device,
                writes.len() as u32,
                writes.as_ptr(),
                copies.len() as u32,
                copies.as_ptr(),
            );
        }
    }

    pub fn free(
        device: VkDevice,
        descriptor_pool: VkDescriptorPool,
        descriptor_sets: &[VkDescriptorSet],
    ) -> Result<(), VkResult> {
        unsafe {
            let result = vkFreeDescriptorSets(
                device,
                descriptor_pool,
                descriptor_sets.len() as u32,
                descriptor_sets.as_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(())
            } else {
                Err(result)
            }
        }
    }
}
*/
