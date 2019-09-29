
use crate::prelude::*;
use crate::SetupU64Conv;

use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkQueryPool(u64);
SetupU64Conv!(VkQueryPool);

/*
impl VkQueryPool {
    pub fn create(
        device: VkDevice,
        query_pool_create_info: &VkQueryPoolCreateInfo,
    ) -> Result<VkQueryPool, VkResult> {
        unsafe {
            let mut query_pool = mem::uninitialized();

            let result =
                vkCreateQueryPool(device, query_pool_create_info, ptr::null(), &mut query_pool);

            if result == VK_SUCCESS {
                Ok(query_pool)
            } else {
                Err(result)
            }
        }
    }

    pub fn create_with_allocation_callbacks(
        device: VkDevice,
        query_pool_create_info: &VkQueryPoolCreateInfo,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<VkQueryPool, VkResult> {
        unsafe {
            let mut query_pool = mem::uninitialized();

            let result = vkCreateQueryPool(
                device,
                query_pool_create_info,
                allocation_callbacks,
                &mut query_pool,
            );

            if result == VK_SUCCESS {
                Ok(query_pool)
            } else {
                Err(result)
            }
        }
    }

    pub fn destroy(&self, device: VkDevice) {
        unsafe { vkDestroyQueryPool(device, *self, ptr::null()) };
    }

    pub fn destroy_with_allocation_callbacks(
        &self,
        device: VkDevice,
        allocation_callbacks: &VkAllocationCallbacks,
    ) {
        unsafe { vkDestroyQueryPool(device, *self, allocation_callbacks) };
    }
}
*/
