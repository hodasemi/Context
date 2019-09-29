
use crate::prelude::*;
use crate::SetupU64Conv;

use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkBufferView(u64);
SetupU64Conv!(VkBufferView);

/*
impl VkBufferView {
    pub fn create(
        device: VkDevice,
        buffer_view_create_info: &VkBufferViewCreateInfo,
    ) -> Result<VkBufferView, VkResult> {
        unsafe {
            let mut buffer_view = mem::uninitialized();

            let result = vkCreateBufferView(
                device,
                buffer_view_create_info,
                ptr::null(),
                &mut buffer_view,
            );

            if result == VK_SUCCESS {
                Ok(buffer_view)
            } else {
                Err(result)
            }
        }
    }

    pub fn create_with_allocation_callbacks(
        device: VkDevice,
        buffer_view_create_info: &VkBufferViewCreateInfo,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<VkBufferView, VkResult> {
        unsafe {
            let mut buffer_view = mem::uninitialized();

            let result = vkCreateBufferView(
                device,
                buffer_view_create_info,
                allocation_callbacks,
                &mut buffer_view,
            );

            if result == VK_SUCCESS {
                Ok(buffer_view)
            } else {
                Err(result)
            }
        }
    }

    pub fn destroy(&self, device: VkDevice) {
        unsafe { vkDestroyBufferView(device, *self, ptr::null()) };
    }

    pub fn destroy_with_allocation_callbacks(
        &self,
        device: VkDevice,
        allocation_callbacks: &VkAllocationCallbacks,
    ) {
        unsafe { vkDestroyBufferView(device, *self, allocation_callbacks) };
    }
}
*/
