
use crate::prelude::*;
use crate::SetupU64Conv;

use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkCommandPool(u64);
SetupU64Conv!(VkCommandPool);

/*
impl VkCommandPool {
    pub fn create(
        device: VkDevice,
        command_pool_layout_create_info: &VkCommandPoolCreateInfo,
    ) -> Result<VkCommandPool, VkResult> {
        unsafe {
            let mut command_pool = mem::uninitialized();

            let result = vkCreateCommandPool(
                device,
                command_pool_layout_create_info,
                ptr::null(),
                &mut command_pool,
            );

            if result == VK_SUCCESS {
                Ok(command_pool)
            } else {
                Err(result)
            }
        }
    }

    pub fn create_with_allocation_callbacks(
        device: VkDevice,
        command_pool_layout_create_info: &VkCommandPoolCreateInfo,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<VkCommandPool, VkResult> {
        unsafe {
            let mut command_pool = mem::uninitialized();

            let result = vkCreateCommandPool(
                device,
                command_pool_layout_create_info,
                allocation_callbacks,
                &mut command_pool,
            );

            if result == VK_SUCCESS {
                Ok(command_pool)
            } else {
                Err(result)
            }
        }
    }

    pub fn reset<T>(&self, device: VkDevice, flags: T) -> Result<(), VkResult>
    where
        T: Into<VkCommandPoolResetFlags>,
    {
        unsafe {
            let result = vkResetCommandPool(device, *self, flags.into());

            if result == VK_SUCCESS {
                Ok(())
            } else {
                Err(result)
            }
        }
    }

    pub fn trim_khr<T>(&self, device: VkDevice, flags: T)
    where
        T: Into<VkCommandPoolTrimFlagsKHR>,
    {
        unsafe {
            vkTrimCommandPoolKHR(device, *self, flags.into());
        }
    }

    pub fn destroy(&self, device: VkDevice) {
        unsafe { vkDestroyCommandPool(device, *self, ptr::null()) };
    }

    pub fn destroy_with_allocation_callbacks(
        &self,
        device: VkDevice,
        allocation_callbacks: &VkAllocationCallbacks,
    ) {
        unsafe { vkDestroyCommandPool(device, *self, allocation_callbacks) };
    }
}
*/
