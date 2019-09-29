
use crate::prelude::*;
use crate::SetupU64Conv;

use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkSemaphore(u64);
SetupU64Conv!(VkSemaphore);

/*
impl VkSemaphore {
    pub fn create(
        device: VkDevice,
        semaphore_create_info: &VkSemaphoreCreateInfo,
    ) -> Result<VkSemaphore, VkResult> {
        unsafe {
            let mut semaphore = mem::uninitialized();

            let result =
                vkCreateSemaphore(device, semaphore_create_info, ptr::null(), &mut semaphore);

            if result == VK_SUCCESS {
                Ok(semaphore)
            } else {
                Err(result)
            }
        }
    }

    pub fn create_with_allocation_callbacks(
        device: VkDevice,
        semaphore_create_info: &VkSemaphoreCreateInfo,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<VkSemaphore, VkResult> {
        unsafe {
            let mut semaphore = mem::uninitialized();

            let result = vkCreateSemaphore(
                device,
                semaphore_create_info,
                allocation_callbacks,
                &mut semaphore,
            );

            if result == VK_SUCCESS {
                Ok(semaphore)
            } else {
                Err(result)
            }
        }
    }

    pub fn destroy(&self, device: VkDevice) {
        unsafe { vkDestroySemaphore(device, *self, ptr::null()) };
    }

    pub fn destroy_with_allocation_callbacks(
        &self,
        device: VkDevice,
        allocation_callbacks: &VkAllocationCallbacks,
    ) {
        unsafe { vkDestroySemaphore(device, *self, allocation_callbacks) };
    }
}
*/
