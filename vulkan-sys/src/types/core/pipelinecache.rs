
use crate::prelude::*;
use crate::SetupU64Conv;

use std::mem;
use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkPipelineCache(u64);
SetupU64Conv!(VkPipelineCache);

/*
impl VkPipelineCache {
    pub fn create(
        device: VkDevice,
        pipeline_cache_create_info: &VkPipelineCacheCreateInfo,
    ) -> Result<VkPipelineCache, VkResult> {
        unsafe {
            let mut pipeline_cache = mem::uninitialized();

            let result = vkCreatePipelineCache(
                device,
                pipeline_cache_create_info,
                ptr::null(),
                &mut pipeline_cache,
            );

            if result == VK_SUCCESS {
                Ok(pipeline_cache)
            } else {
                Err(result)
            }
        }
    }

    pub fn create_with_allocation_callbacks(
        device: VkDevice,
        pipeline_cache_create_info: &VkPipelineCacheCreateInfo,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<VkPipelineCache, VkResult> {
        unsafe {
            let mut pipeline_cache = mem::uninitialized();

            let result = vkCreatePipelineCache(
                device,
                pipeline_cache_create_info,
                allocation_callbacks,
                &mut pipeline_cache,
            );

            if result == VK_SUCCESS {
                Ok(pipeline_cache)
            } else {
                Err(result)
            }
        }
    }

    pub fn get_data<T>(&self, device: VkDevice) -> Result<T, VkResult> {
        let mut count = 0;

        let result = unsafe { vkGetPipelineCacheData(device, *self, &mut count, ptr::null_mut()) };

        if result != VK_SUCCESS {
            return Err(result);
        }

        if count != mem::size_of::<T>() {
            // improvised error type
            return Err(VK_INCOMPLETE);
        }

        let mut data: T = unsafe { mem::uninitialized() };

        let result = unsafe {
            vkGetPipelineCacheData(
                device,
                *self,
                &mut count,
                &mut data as *mut T as *mut c_void,
            )
        };

        if result == VK_SUCCESS {
            Ok(data)
        } else {
            Err(result)
        }
    }

    pub fn merge(&self, device: VkDevice, src_caches: &[VkPipelineCache]) -> Result<(), VkResult> {
        unsafe {
            let result =
                vkMergePipelineCaches(device, *self, src_caches.len() as u32, src_caches.as_ptr());

            if result == VK_SUCCESS {
                Ok(())
            } else {
                Err(result)
            }
        }
    }

    pub fn destroy(&self, device: VkDevice) {
        unsafe { vkDestroyPipelineCache(device, *self, ptr::null()) };
    }

    pub fn destroy_with_allocation_callbacks(
        &self,
        device: VkDevice,
        allocation_callbacks: &VkAllocationCallbacks,
    ) {
        unsafe { vkDestroyPipelineCache(device, *self, allocation_callbacks) };
    }
}
*/
