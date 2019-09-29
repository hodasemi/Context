
use crate::prelude::*;
use crate::SetupU64Conv;

use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkSampler(u64);
SetupU64Conv!(VkSampler);

/*
impl VkSampler {
    pub fn create(
        device: VkDevice,
        sampler_create_info: &VkSamplerCreateInfo,
    ) -> Result<VkSampler, VkResult> {
        unsafe {
            let mut sampler = mem::uninitialized();

            let result = vkCreateSampler(device, sampler_create_info, ptr::null(), &mut sampler);

            if result == VK_SUCCESS {
                Ok(sampler)
            } else {
                Err(result)
            }
        }
    }

    pub fn create_with_allocation_callbacks(
        device: VkDevice,
        sampler_create_info: &VkSamplerCreateInfo,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<VkSampler, VkResult> {
        unsafe {
            let mut sampler = mem::uninitialized();

            let result = vkCreateSampler(
                device,
                sampler_create_info,
                allocation_callbacks,
                &mut sampler,
            );

            if result == VK_SUCCESS {
                Ok(sampler)
            } else {
                Err(result)
            }
        }
    }

    pub fn destroy(&self, device: VkDevice) {
        unsafe { vkDestroySampler(device, *self, ptr::null()) };
    }

    pub fn destroy_with_allocation_callbacks(
        &self,
        device: VkDevice,
        allocation_callbacks: &VkAllocationCallbacks,
    ) {
        unsafe { vkDestroySampler(device, *self, allocation_callbacks) };
    }
}
*/
