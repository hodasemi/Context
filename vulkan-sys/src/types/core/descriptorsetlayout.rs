
use crate::prelude::*;
use crate::SetupU64Conv;

use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkDescriptorSetLayout(u64);
SetupU64Conv!(VkDescriptorSetLayout);

/*
impl VkDescriptorSetLayout {
    pub fn create(
        device: VkDevice,
        descriptor_set_layout_create_info: &VkDescriptorSetLayoutCreateInfo,
    ) -> Result<VkDescriptorSetLayout, VkResult> {
        unsafe {
            let mut descriptor_set_layout = mem::uninitialized();

            let result = vkCreateDescriptorSetLayout(
                device,
                descriptor_set_layout_create_info,
                ptr::null(),
                &mut descriptor_set_layout,
            );

            if result == VK_SUCCESS {
                Ok(descriptor_set_layout)
            } else {
                Err(result)
            }
        }
    }

    pub fn create_with_allocation_callbacks(
        device: VkDevice,
        descriptor_set_layout_create_info: &VkDescriptorSetLayoutCreateInfo,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<VkDescriptorSetLayout, VkResult> {
        unsafe {
            let mut descriptor_set_layout = mem::uninitialized();

            let result = vkCreateDescriptorSetLayout(
                device,
                descriptor_set_layout_create_info,
                allocation_callbacks,
                &mut descriptor_set_layout,
            );

            if result == VK_SUCCESS {
                Ok(descriptor_set_layout)
            } else {
                Err(result)
            }
        }
    }

    pub fn destroy(&self, device: VkDevice) {
        unsafe { vkDestroyDescriptorSetLayout(device, *self, ptr::null()) };
    }

    pub fn destroy_with_allocation_callbacks(
        &self,
        device: VkDevice,
        allocation_callbacks: &VkAllocationCallbacks,
    ) {
        unsafe { vkDestroyDescriptorSetLayout(device, *self, allocation_callbacks) };
    }
}
*/
