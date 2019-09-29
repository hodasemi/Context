
use crate::prelude::*;
use crate::SetupU64Conv;

use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkEvent(u64);
SetupU64Conv!(VkEvent);

/*
impl VkEvent {
    pub fn create(
        device: VkDevice,
        event_create_info: &VkEventCreateInfo,
    ) -> Result<VkEvent, VkResult> {
        unsafe {
            let mut event = mem::uninitialized();

            let result = vkCreateEvent(device, event_create_info, ptr::null(), &mut event);

            if result == VK_SUCCESS {
                Ok(event)
            } else {
                Err(result)
            }
        }
    }

    pub fn create_with_allocation_callbacks(
        device: VkDevice,
        event_create_info: &VkEventCreateInfo,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<VkEvent, VkResult> {
        unsafe {
            let mut event = mem::uninitialized();

            let result = vkCreateEvent(device, event_create_info, allocation_callbacks, &mut event);

            if result == VK_SUCCESS {
                Ok(event)
            } else {
                Err(result)
            }
        }
    }

    pub fn get_status(&self, device: VkDevice) -> Result<(), VkResult> {
        unsafe {
            let result = vkGetEventStatus(device, *self);

            if result == VK_SUCCESS {
                Ok(())
            } else {
                Err(result)
            }
        }
    }

    pub fn set(&self, device: VkDevice) -> Result<(), VkResult> {
        unsafe {
            let result = vkSetEvent(device, *self);

            if result == VK_SUCCESS {
                Ok(())
            } else {
                Err(result)
            }
        }
    }

    pub fn reset(&self, device: VkDevice) -> Result<(), VkResult> {
        unsafe {
            let result = vkResetEvent(device, *self);

            if result == VK_SUCCESS {
                Ok(())
            } else {
                Err(result)
            }
        }
    }

    pub fn destroy(&self, device: VkDevice) {
        unsafe { vkDestroyEvent(device, *self, ptr::null()) };
    }

    pub fn destroy_with_allocation_callbacks(
        &self,
        device: VkDevice,
        allocation_callbacks: &VkAllocationCallbacks,
    ) {
        unsafe { vkDestroyEvent(device, *self, allocation_callbacks) };
    }
}
*/
