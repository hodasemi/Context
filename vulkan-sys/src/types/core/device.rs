
use crate::prelude::*;
use crate::SetupUSizeConv;

use core::ffi::c_void;
use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkDevice(usize);
SetupUSizeConv!(VkDevice);

/*
impl VkDevice {
    pub fn create(
        physical_device: VkPhysicalDevice,
        device_create_info: &VkDeviceCreateInfo,
    ) -> Result<VkDevice, VkResult> {
        unsafe {
            let mut device = mem::uninitialized();
            let result = vkCreateDevice(
                physical_device,
                device_create_info,
                ptr::null(),
                &mut device,
            );

            if result == VK_SUCCESS {
                Ok(device)
            } else {
                Err(result)
            }
        }
    }

    pub fn create_with_allocation_callbacks(
        physical_device: VkPhysicalDevice,
        device_create_info: &VkDeviceCreateInfo,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<VkDevice, VkResult> {
        unsafe {
            let mut device = mem::uninitialized();
            let result = vkCreateDevice(
                physical_device,
                device_create_info,
                allocation_callbacks,
                &mut device,
            );

            if result == VK_SUCCESS {
                Ok(device)
            } else {
                Err(result)
            }
        }
    }

    pub fn get_queue(&self, queue_family_index: u32, queue_index: u32) -> VkQueue {
        unsafe {
            let mut queue = mem::uninitialized();
            vkGetDeviceQueue(*self, queue_family_index, queue_index, &mut queue);

            queue
        }
    }

    pub fn wait_for_fences<T>(
        &self,
        fences: &[VkFence],
        wait_all: T,
        timeout: u64,
    ) -> Result<(), VkResult>
    where
        T: Into<VkBool32>,
    {
        unsafe {
            let result = vkWaitForFences(
                *self,
                fences.len() as u32,
                fences.as_ptr(),
                wait_all.into(),
                timeout,
            );

            if result == VK_SUCCESS {
                Ok(())
            } else {
                Err(result)
            }
        }
    }

    pub fn get_query_pool_results<T, U>(
        &self,
        query_pool: VkQueryPool,
        first_query: u32,
        query_count: u32,
        data: &mut T,
        stride: VkDeviceSize,
        flags: U,
    ) -> Result<(), VkResult>
    where
        U: Into<VkQueryResultFlagBits>,
    {
        unsafe {
            let result = vkGetQueryPoolResults(
                *self,
                query_pool,
                first_query,
                query_count,
                mem::size_of::<T>(),
                data as *mut T as *mut c_void,
                stride,
                flags.into(),
            );

            if result == VK_SUCCESS {
                Ok(())
            } else {
                Err(result)
            }
        }
    }

    pub fn get_proc_addr(&self, name: VkString) -> Option<PFN_vkVoidFunction> {
        Some(unsafe { vkGetDeviceProcAddr(*self, name.as_ptr()) })
    }

    pub fn get_acceleration_structure_memory_requirements(
        &self,
        info: &VkAccelerationStructureMemoryRequirementsInfoNV,
    ) -> Result<VkMemoryRequirements2KHR, VkResult> {
        unsafe {
            match vkGetAccelerationStructureMemoryRequirementsNV {
                Some(pfn) => {
                    let mut memory_requirements = mem::uninitialized();

                    pfn(*self, info, &mut memory_requirements);

                    Ok(memory_requirements)
                }
                None => return Err(VK_ERROR_EXTENSION_NOT_PRESENT),
            }
        }
    }

    pub fn bind_acceleration_structure_memory(
        &self,
        bind_infos: &[VkBindAccelerationStructureMemoryInfoNV],
    ) -> Result<(), VkResult> {
        unsafe {
            match vkBindAccelerationStructureMemoryNV {
                Some(pfn) => {
                    pfn(*self, bind_infos.len() as u32, bind_infos.as_ptr());

                    Ok(())
                }
                None => return Err(VK_ERROR_EXTENSION_NOT_PRESENT),
            }
        }
    }

    pub fn destroy(&self) {
        unsafe { vkDestroyDevice(*self, ptr::null()) }
    }

    pub fn destroy_with_allocation_callbacks(&self, allocation_callbacks: &VkAllocationCallbacks) {
        unsafe { vkDestroyDevice(*self, allocation_callbacks) }
    }
}
*/
