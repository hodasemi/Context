
use crate::prelude::*;
use crate::SetupU64Conv;

use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkFence(u64);
SetupU64Conv!(VkFence);

/*
impl VkFence {
    pub fn create(
        device: VkDevice,
        fence_create_info: &VkFenceCreateInfo,
    ) -> Result<VkFence, VkResult> {
        unsafe {
            let mut fence = mem::uninitialized();

            let result = vkCreateFence(device, fence_create_info, ptr::null(), &mut fence);

            if result == VK_SUCCESS {
                Ok(fence)
            } else {
                Err(result)
            }
        }
    }

    pub fn create_with_allocation_callbacks(
        device: VkDevice,
        fence_create_info: &VkFenceCreateInfo,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<VkFence, VkResult> {
        unsafe {
            let mut fence = mem::uninitialized();

            let result = vkCreateFence(device, fence_create_info, allocation_callbacks, &mut fence);

            if result == VK_SUCCESS {
                Ok(fence)
            } else {
                Err(result)
            }
        }
    }

    pub fn reset(device: VkDevice, fences: &[VkFence]) -> Result<(), VkResult> {
        unsafe {
            let result = vkResetFences(device, fences.len() as u32, fences.as_ptr());

            if result == VK_SUCCESS {
                Ok(())
            } else {
                Err(result)
            }
        }
    }

    pub fn destroy(&self, device: VkDevice) {
        unsafe { vkDestroyFence(device, *self, ptr::null()) };
    }

    pub fn destroy_with_allocation_callbacks(
        &self,
        device: VkDevice,
        allocation_callbacks: &VkAllocationCallbacks,
    ) {
        unsafe { vkDestroyFence(device, *self, allocation_callbacks) };
    }
}
*/
