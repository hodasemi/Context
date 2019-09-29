
use crate::prelude::*;
use crate::SetupU64Conv;

use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkBuffer(u64);
SetupU64Conv!(VkBuffer);

/*
impl VkBuffer {
    pub fn create(
        device: VkDevice,
        buffer_create_info: &VkBufferCreateInfo,
    ) -> Result<VkBuffer, VkResult> {
        unsafe {
            let mut buffer = mem::uninitialized();
            let result = vkCreateBuffer(device, buffer_create_info, ptr::null(), &mut buffer);

            if result == VK_SUCCESS {
                Ok(buffer)
            } else {
                Err(result)
            }
        }
    }
    pub fn create_with_allocation_callbacks(
        device: VkDevice,
        buffer_create_info: &VkBufferCreateInfo,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<VkBuffer, VkResult> {
        unsafe {
            let mut buffer = mem::uninitialized();
            let result = vkCreateBuffer(
                device,
                buffer_create_info,
                allocation_callbacks,
                &mut buffer,
            );

            if result == VK_SUCCESS {
                Ok(buffer)
            } else {
                Err(result)
            }
        }
    }

    pub fn get_memory_requirements(&self, device: VkDevice) -> VkMemoryRequirements {
        unsafe {
            let mut memory_requirements = mem::uninitialized();

            vkGetBufferMemoryRequirements(device, *self, &mut memory_requirements);

            memory_requirements
        }
    }

    pub fn bind_memory(
        &self,
        device: VkDevice,
        memory: VkDeviceMemory,
        offset: VkDeviceSize,
    ) -> Result<(), VkResult> {
        unsafe {
            let result = vkBindBufferMemory(device, *self, memory, offset);

            if result == VK_SUCCESS {
                Ok(())
            } else {
                Err(result)
            }
        }
    }

    pub fn destroy(&self, device: VkDevice) {
        unsafe { vkDestroyBuffer(device, *self, ptr::null()) };
    }

    pub fn destroy_with_allocation_callbacks(
        &self,
        device: VkDevice,
        allocation_callbacks: &VkAllocationCallbacks,
    ) {
        unsafe { vkDestroyBuffer(device, *self, allocation_callbacks) };
    }
}
*/
