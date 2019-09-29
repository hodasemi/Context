
use crate::prelude::*;
use crate::SetupU64Conv;

use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkSwapchainKHR(u64);
SetupU64Conv!(VkSwapchainKHR);

/*
impl VkSwapchainKHR {
    pub fn create(
        device: VkDevice,
        swapchain_create_info: &VkSwapchainCreateInfoKHR,
    ) -> Result<VkSwapchainKHR, VkResult> {
        unsafe {
            let mut swapchain = mem::uninitialized();

            let result =
                vkCreateSwapchainKHR(device, swapchain_create_info, ptr::null(), &mut swapchain);

            if result == VK_SUCCESS {
                Ok(swapchain)
            } else {
                Err(result)
            }
        }
    }

    pub fn create_with_allocation_callbacks(
        device: VkDevice,
        swapchain_create_info: &VkSwapchainCreateInfoKHR,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<VkSwapchainKHR, VkResult> {
        unsafe {
            let mut swapchain = mem::uninitialized();

            let result = vkCreateSwapchainKHR(
                device,
                swapchain_create_info,
                allocation_callbacks,
                &mut swapchain,
            );

            if result == VK_SUCCESS {
                Ok(swapchain)
            } else {
                Err(result)
            }
        }
    }

    pub fn get_images(&self, device: VkDevice) -> Result<Vec<VkImage>, VkResult> {
        let mut count = 0;

        let result = unsafe { vkGetSwapchainImagesKHR(device, *self, &mut count, ptr::null_mut()) };

        if result != VK_SUCCESS {
            return Err(result);
        }

        let mut images = Vec::with_capacity(count as usize);
        unsafe { images.set_len(count as usize) };

        let result =
            unsafe { vkGetSwapchainImagesKHR(device, *self, &mut count, images.as_mut_ptr()) };

        if result == VK_SUCCESS {
            Ok(images)
        } else {
            Err(result)
        }
    }

    pub fn acquire_next_image(
        &self,
        device: VkDevice,
        timeout: u64,
        semaphore: Option<VkSemaphore>,
        fence: Option<VkFence>,
    ) -> Result<u32, VkResult> {
        unsafe {
            let mut image_index = 0;

            let result = vkAcquireNextImageKHR(
                device,
                *self,
                timeout,
                match semaphore {
                    Some(sem) => sem,
                    None => VkSemaphore::default(),
                },
                match fence {
                    Some(fence) => fence,
                    None => VkFence::default(),
                },
                &mut image_index,
            );

            if result == VK_SUCCESS {
                Ok(image_index)
            } else {
                Err(result)
            }
        }
    }

    pub fn destroy(&self, device: VkDevice) {
        unsafe { vkDestroySwapchainKHR(device, *self, ptr::null()) };
    }

    pub fn destroy_with_allocation_callbacks(
        &self,
        device: VkDevice,
        allocation_callbacks: &VkAllocationCallbacks,
    ) {
        unsafe { vkDestroySwapchainKHR(device, *self, allocation_callbacks) };
    }
}
*/
