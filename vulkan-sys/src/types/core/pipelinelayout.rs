
use crate::prelude::*;
use crate::SetupU64Conv;

use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkPipelineLayout(u64);
SetupU64Conv!(VkPipelineLayout);

/*
impl VkPipelineLayout {
    pub fn create(
        device: VkDevice,
        pipeline_layout_create_info: &VkPipelineLayoutCreateInfo,
    ) -> Result<VkPipelineLayout, VkResult> {
        unsafe {
            let mut pipeline_layout = mem::uninitialized();

            let result = vkCreatePipelineLayout(
                device,
                pipeline_layout_create_info,
                ptr::null(),
                &mut pipeline_layout,
            );

            if result == VK_SUCCESS {
                Ok(pipeline_layout)
            } else {
                Err(result)
            }
        }
    }

    pub fn create_with_allocation_callbacks(
        device: VkDevice,
        pipeline_layout_create_info: &VkPipelineLayoutCreateInfo,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<VkPipelineLayout, VkResult> {
        unsafe {
            let mut pipeline_layout = mem::uninitialized();

            let result = vkCreatePipelineLayout(
                device,
                pipeline_layout_create_info,
                allocation_callbacks,
                &mut pipeline_layout,
            );

            if result == VK_SUCCESS {
                Ok(pipeline_layout)
            } else {
                Err(result)
            }
        }
    }

    pub fn destroy(&self, device: VkDevice) {
        unsafe { vkDestroyPipelineLayout(device, *self, ptr::null()) };
    }

    pub fn destroy_with_allocation_callbacks(
        &self,
        device: VkDevice,
        allocation_callbacks: &VkAllocationCallbacks,
    ) {
        unsafe { vkDestroyPipelineLayout(device, *self, allocation_callbacks) };
    }
}
*/
