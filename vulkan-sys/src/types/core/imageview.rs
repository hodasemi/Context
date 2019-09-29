
use crate::prelude::*;
use crate::SetupU64Conv;

use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkImageView(u64);
SetupU64Conv!(VkImageView);

/*
impl VkImageView {
    pub fn create(
        device: VkDevice,
        image_view_create_info: &VkImageViewCreateInfo,
    ) -> Result<VkImageView, VkResult> {
        unsafe {
            let mut image_view = mem::uninitialized();

            let result =
                vkCreateImageView(device, image_view_create_info, ptr::null(), &mut image_view);

            if result == VK_SUCCESS {
                Ok(image_view)
            } else {
                Err(result)
            }
        }
    }

    pub fn create_with_allocation_callbacks(
        device: VkDevice,
        image_view_create_info: &VkImageViewCreateInfo,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<VkImageView, VkResult> {
        unsafe {
            let mut image_view = mem::uninitialized();

            let result = vkCreateImageView(
                device,
                image_view_create_info,
                allocation_callbacks,
                &mut image_view,
            );

            if result == VK_SUCCESS {
                Ok(image_view)
            } else {
                Err(result)
            }
        }
    }

    pub fn destroy(&self, device: VkDevice) {
        unsafe { vkDestroyImageView(device, *self, ptr::null()) };
    }

    pub fn destroy_with_allocation_callbacks(
        &self,
        device: VkDevice,
        allocation_callbacks: &VkAllocationCallbacks,
    ) {
        unsafe { vkDestroyImageView(device, *self, allocation_callbacks) };
    }
}
*/
