
use crate::prelude::*;
use crate::SetupU64Conv;

use std::mem;
use std::ptr;
use std::slice;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkDeviceMemory(u64);
SetupU64Conv!(VkDeviceMemory);

/*
impl VkDeviceMemory {
    pub fn allocate(
        device: VkDevice,
        allocate_info: &VkMemoryAllocateInfo,
    ) -> Result<VkDeviceMemory, VkResult> {
        unsafe {
            let mut memory = mem::uninitialized();

            let result = vkAllocateMemory(device, allocate_info, ptr::null(), &mut memory);

            if result == VK_SUCCESS {
                Ok(memory)
            } else {
                Err(result)
            }
        }
    }

    pub fn allocate_with_allocation_callbacks(
        device: VkDevice,
        allocate_info: &VkMemoryAllocateInfo,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<VkDeviceMemory, VkResult> {
        unsafe {
            let mut memory = mem::uninitialized();

            let result = vkAllocateMemory(device, allocate_info, allocation_callbacks, &mut memory);

            if result == VK_SUCCESS {
                Ok(memory)
            } else {
                Err(result)
            }
        }
    }

    pub fn map<T, U>(
        &self,
        device: VkDevice,
        offset: VkDeviceSize,
        length: VkDeviceSize,
        flags: T,
    ) -> Result<VkMappedMemory<'_, U>, VkResult>
    where
        T: Into<VkMemoryMapFlags>,
        U: Copy,
    {
        unsafe {
            let mut data = mem::uninitialized();

            let size = length * mem::size_of::<U>() as VkDeviceSize;

            let result = vkMapMemory(device, *self, offset, size, flags.into(), &mut data);

            if result == VK_SUCCESS {
                let slice = slice::from_raw_parts_mut(data as *mut U, length as usize);
                Ok(VkMappedMemory::new(device, *self, slice))
            } else {
                Err(result)
            }
        }
    }

    pub fn unmap(&self, device: VkDevice) {
        unsafe { vkUnmapMemory(device, *self) };
    }

    pub fn free(&self, device: VkDevice) {
        unsafe { vkFreeMemory(device, *self, ptr::null()) };
    }

    pub fn free_with_allocation_callbacks(
        &self,
        device: VkDevice,
        allocation_callbacks: &VkAllocationCallbacks,
    ) {
        unsafe { vkFreeMemory(device, *self, allocation_callbacks) };
    }
}
*/
