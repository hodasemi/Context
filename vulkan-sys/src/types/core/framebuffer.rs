
use crate::prelude::*;
use crate::SetupU64Conv;

use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkFramebuffer(u64);
SetupU64Conv!(VkFramebuffer);

/*
impl VkFramebuffer {
    pub fn create(
        device: VkDevice,
        framebuffer_create_info: &VkFramebufferCreateInfo,
    ) -> Result<VkFramebuffer, VkResult> {
        unsafe {
            let mut framebuffer = mem::uninitialized();

            let result = vkCreateFramebuffer(
                device,
                framebuffer_create_info,
                ptr::null(),
                &mut framebuffer,
            );

            if result == VK_SUCCESS {
                Ok(framebuffer)
            } else {
                Err(result)
            }
        }
    }

    pub fn create_with_allocation_callbacks(
        device: VkDevice,
        framebuffer_create_info: &VkFramebufferCreateInfo,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<VkFramebuffer, VkResult> {
        unsafe {
            let mut framebuffer = mem::uninitialized();

            let result = vkCreateFramebuffer(
                device,
                framebuffer_create_info,
                allocation_callbacks,
                &mut framebuffer,
            );

            if result == VK_SUCCESS {
                Ok(framebuffer)
            } else {
                Err(result)
            }
        }
    }

    pub fn destroy(&self, device: VkDevice) {
        unsafe { vkDestroyFramebuffer(device, *self, ptr::null()) };
    }

    pub fn destroy_with_allocation_callbacks(
        &self,
        device: VkDevice,
        allocation_callbacks: &VkAllocationCallbacks,
    ) {
        unsafe { vkDestroyFramebuffer(device, *self, allocation_callbacks) };
    }
}
*/
