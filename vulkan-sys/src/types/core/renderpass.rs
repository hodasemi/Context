
use crate::prelude::*;
use crate::SetupU64Conv;

use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkRenderPass(u64);
SetupU64Conv!(VkRenderPass);

/*
impl VkRenderPass {
    pub fn create(
        device: VkDevice,
        render_pass_create_info: &VkRenderPassCreateInfo,
    ) -> Result<VkRenderPass, VkResult> {
        unsafe {
            let mut render_pass = mem::uninitialized();

            let result = vkCreateRenderPass(
                device,
                render_pass_create_info,
                ptr::null(),
                &mut render_pass,
            );

            if result == VK_SUCCESS {
                Ok(render_pass)
            } else {
                Err(result)
            }
        }
    }

    pub fn create_with_allocation_callbacks(
        device: VkDevice,
        render_pass_create_info: &VkRenderPassCreateInfo,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<VkRenderPass, VkResult> {
        unsafe {
            let mut render_pass = mem::uninitialized();

            let result = vkCreateRenderPass(
                device,
                render_pass_create_info,
                allocation_callbacks,
                &mut render_pass,
            );

            if result == VK_SUCCESS {
                Ok(render_pass)
            } else {
                Err(result)
            }
        }
    }

    pub fn destroy(&self, device: VkDevice) {
        unsafe { vkDestroyRenderPass(device, *self, ptr::null()) };
    }

    pub fn destroy_with_allocation_callbacks(
        &self,
        device: VkDevice,
        allocation_callbacks: &VkAllocationCallbacks,
    ) {
        unsafe { vkDestroyRenderPass(device, *self, allocation_callbacks) };
    }
}
*/
