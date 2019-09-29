use crate::prelude::*;
use crate::SetupU64Conv;

use core::ffi::c_void;
use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkAccelerationStructureNV(u64);
SetupU64Conv!(VkAccelerationStructureNV);

/*
impl VkAccelerationStructureNV {
    pub fn create(
        device: VkDevice,
        acceleration_structure_create_info: &VkAccelerationStructureCreateInfoNV,
    ) -> Result<VkAccelerationStructureNV, VkResult> {
        unsafe {
            match vkCreateAccelerationStructureNV {
                Some(pfn) => {
                    let mut acceleration_structure = mem::uninitialized();

                    let result = pfn(
                        device,
                        acceleration_structure_create_info,
                        ptr::null(),
                        &mut acceleration_structure,
                    );

                    if result == VK_SUCCESS {
                        Ok(acceleration_structure)
                    } else {
                        Err(result)
                    }
                }
                None => return Err(VK_ERROR_EXTENSION_NOT_PRESENT),
            }
        }
    }

    pub fn create_with_allocation_callbacks(
        device: VkDevice,
        acceleration_structure_create_info: &VkAccelerationStructureCreateInfoNV,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<VkAccelerationStructureNV, VkResult> {
        unsafe {
            match vkCreateAccelerationStructureNV {
                Some(pfn) => {
                    let mut acceleration_structure = mem::uninitialized();

                    let result = pfn(
                        device,
                        acceleration_structure_create_info,
                        allocation_callbacks,
                        &mut acceleration_structure,
                    );

                    if result == VK_SUCCESS {
                        Ok(acceleration_structure)
                    } else {
                        Err(result)
                    }
                }
                None => return Err(VK_ERROR_EXTENSION_NOT_PRESENT),
            }
        }
    }

    pub fn get_handle(&self, device: VkDevice) -> Result<u64, VkResult> {
        unsafe {
            match vkGetAccelerationStructureHandleNV {
                Some(pfn) => {
                    let mut handle = 0;

                    let result = pfn(
                        device,
                        *self,
                        mem::size_of::<u64>(),
                        &mut handle as *mut u64 as *mut c_void,
                    );

                    if result == VK_SUCCESS {
                        Ok(handle)
                    } else {
                        Err(result)
                    }
                }
                None => return Err(VK_ERROR_EXTENSION_NOT_PRESENT),
            }
        }
    }

    pub fn destroy(&self, device: VkDevice) -> Result<(), VkResult> {
        unsafe {
            match vkDestroyAccelerationStructureNV {
                Some(pfn) => {
                    pfn(device, *self, ptr::null());

                    Ok(())
                }
                None => return Err(VK_ERROR_EXTENSION_NOT_PRESENT),
            }
        }
    }

    pub fn destroy_with_allocation_callbacks(
        &self,
        device: VkDevice,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<(), VkResult> {
        unsafe {
            match vkDestroyAccelerationStructureNV {
                Some(pfn) => {
                    pfn(device, *self, allocation_callbacks);

                    Ok(())
                }
                None => return Err(VK_ERROR_EXTENSION_NOT_PRESENT),
            }
        }
    }
}
*/
