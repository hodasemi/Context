use crate::load_function_ptrs;
use crate::prelude::*;

use std::os::raw::c_char;
use std::os::raw::c_void;

load_function_ptrs!(DeviceWSIFunctions, {
    vkQueuePresentKHR(queue: VkQueue, pPresentInfo: *const VkPresentInfoKHR) -> VkResult,

    vkCreateSwapchainKHR(
        device: VkDevice,
        pCreateInfo: *const VkSwapchainCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSwapchain: *mut VkSwapchainKHR
    ) -> VkResult,

    vkDestroySwapchainKHR(
        device: VkDevice,
        swapchain: VkSwapchainKHR,
        pAllocator: *const VkAllocationCallbacks
    ) -> (),

    vkGetSwapchainImagesKHR(
        device: VkDevice,
        swapchain: VkSwapchainKHR,
        pSwapchainImageCount: *mut u32,
        pSwapchainImages: *mut VkImage
    ) -> VkResult,

    vkAcquireNextImageKHR(
        device: VkDevice,
        swapchain: VkSwapchainKHR,
        timeout: u64,
        semaphore: VkSemaphore,
        fence: VkFence,
        pImageIndex: *mut u32
    ) -> VkResult,
});
