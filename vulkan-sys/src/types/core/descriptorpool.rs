
use crate::prelude::*;
use crate::SetupU64Conv;

use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkDescriptorPool(u64);
SetupU64Conv!(VkDescriptorPool);

/*
impl VkDescriptorPool {
    pub fn create(
        device: VkDevice,
        descriptor_pool_create_info: &VkDescriptorPoolCreateInfo,
    ) -> Result<VkDescriptorPool, VkResult> {
        unsafe {
            let mut descriptor_pool = mem::uninitialized();

            let result = vkCreateDescriptorPool(
                device,
                descriptor_pool_create_info,
                ptr::null(),
                &mut descriptor_pool,
            );

            if result == VK_SUCCESS {
                Ok(descriptor_pool)
            } else {
                Err(result)
            }
        }
    }

    pub fn create_with_allocation_callbacks(
        device: VkDevice,
        descriptor_pool_create_info: &VkDescriptorPoolCreateInfo,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<VkDescriptorPool, VkResult> {
        unsafe {
            let mut descriptor_pool = mem::uninitialized();

            let result = vkCreateDescriptorPool(
                device,
                descriptor_pool_create_info,
                allocation_callbacks,
                &mut descriptor_pool,
            );

            if result == VK_SUCCESS {
                Ok(descriptor_pool)
            } else {
                Err(result)
            }
        }
    }

    pub fn reset<T>(&self, device: VkDevice, reset_flags: T) -> Result<(), VkResult>
    where
        T: Into<VkDescriptorPoolResetFlags>,
    {
        unsafe {
            let result = vkResetDescriptorPool(device, *self, reset_flags.into());

            if result == VK_SUCCESS {
                Ok(())
            } else {
                Err(result)
            }
        }
    }

    pub fn destroy(&self, device: VkDevice) {
        unsafe { vkDestroyDescriptorPool(device, *self, ptr::null()) };
    }

    pub fn destroy_with_allocation_callbacks(
        &self,
        device: VkDevice,
        allocation_callbacks: &VkAllocationCallbacks,
    ) {
        unsafe { vkDestroyDescriptorPool(device, *self, allocation_callbacks) };
    }
}
*/
