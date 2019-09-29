
use crate::prelude::*;
use crate::SetupU64Conv;

use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkImage(u64);
SetupU64Conv!(VkImage);

/*
impl VkImage {
    pub fn create(
        device: VkDevice,
        image_create_info: &VkImageCreateInfo,
    ) -> Result<VkImage, VkResult> {
        unsafe {
            let mut image = mem::uninitialized();

            let result = vkCreateImage(device, image_create_info, ptr::null(), &mut image);

            if result == VK_SUCCESS {
                Ok(image)
            } else {
                Err(result)
            }
        }
    }

    pub fn create_with_allocation_callbacks(
        device: VkDevice,
        image_create_info: &VkImageCreateInfo,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<VkImage, VkResult> {
        unsafe {
            let mut image = mem::uninitialized();

            let result = vkCreateImage(device, image_create_info, allocation_callbacks, &mut image);

            if result == VK_SUCCESS {
                Ok(image)
            } else {
                Err(result)
            }
        }
    }

    pub fn get_subresource_layout(
        &self,
        device: VkDevice,
        subresource: &VkImageSubresource,
    ) -> VkSubresourceLayout {
        unsafe {
            let mut subresource_layout = mem::uninitialized();

            vkGetImageSubresourceLayout(device, *self, subresource, &mut subresource_layout);

            subresource_layout
        }
    }

    pub fn get_memory_requirements(&self, device: VkDevice) -> VkMemoryRequirements {
        unsafe {
            let mut memory_requirements = mem::uninitialized();

            vkGetImageMemoryRequirements(device, *self, &mut memory_requirements);

            memory_requirements
        }
    }

    pub fn get_sparse_memory_requirements(
        &self,
        device: VkDevice,
    ) -> Vec<VkSparseImageMemoryRequirements> {
        let mut count: u32 = 0;

        unsafe { vkGetImageSparseMemoryRequirements(device, *self, &mut count, ptr::null_mut()) };

        let mut sparse_memory_requirements = Vec::with_capacity(count as usize);
        unsafe { sparse_memory_requirements.set_len(count as usize) };

        unsafe {
            vkGetImageSparseMemoryRequirements(
                device,
                *self,
                &mut count,
                sparse_memory_requirements.as_mut_ptr(),
            )
        };

        sparse_memory_requirements
    }

    pub fn bind_memory(
        &self,
        device: VkDevice,
        memory: VkDeviceMemory,
        offset: VkDeviceSize,
    ) -> Result<(), VkResult> {
        unsafe {
            let result = vkBindImageMemory(device, *self, memory, offset);

            if result == VK_SUCCESS {
                Ok(())
            } else {
                Err(result)
            }
        }
    }

    pub fn destroy(&self, device: VkDevice) {
        unsafe { vkDestroyImage(device, *self, ptr::null()) };
    }

    pub fn destroy_with_allocation_callbacks(
        &self,
        device: VkDevice,
        allocation_callbacks: &VkAllocationCallbacks,
    ) {
        unsafe { vkDestroyImage(device, *self, allocation_callbacks) };
    }
}
*/
