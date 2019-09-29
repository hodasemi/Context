
use crate::prelude::*;
use crate::SetupU64Conv;

use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkShaderModule(u64);
SetupU64Conv!(VkShaderModule);

/*
impl VkShaderModule {
    pub fn create(
        device: VkDevice,
        shader_module_create_info: &VkShaderModuleCreateInfo,
    ) -> Result<VkShaderModule, VkResult> {
        unsafe {
            let mut shader_module = mem::uninitialized();

            let result = vkCreateShaderModule(
                device,
                shader_module_create_info,
                ptr::null(),
                &mut shader_module,
            );

            if result == VK_SUCCESS {
                Ok(shader_module)
            } else {
                Err(result)
            }
        }
    }

    pub fn create_with_allocation_callbacks(
        device: VkDevice,
        shader_module_create_info: &VkShaderModuleCreateInfo,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<VkShaderModule, VkResult> {
        unsafe {
            let mut shader_module = mem::uninitialized();

            let result = vkCreateShaderModule(
                device,
                shader_module_create_info,
                allocation_callbacks,
                &mut shader_module,
            );

            if result == VK_SUCCESS {
                Ok(shader_module)
            } else {
                Err(result)
            }
        }
    }

    pub fn destroy(&self, device: VkDevice) {
        unsafe { vkDestroyShaderModule(device, *self, ptr::null()) };
    }

    pub fn destroy_with_allocation_callbacks(
        &self,
        device: VkDevice,
        allocation_callbacks: &VkAllocationCallbacks,
    ) {
        unsafe { vkDestroyShaderModule(device, *self, allocation_callbacks) };
    }
}
*/
