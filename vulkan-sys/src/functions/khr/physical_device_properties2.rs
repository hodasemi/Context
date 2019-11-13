use crate::load_function_ptrs;
use crate::prelude::*;

use std::os::raw::c_void;

load_function_ptrs!(PhysicalDeviceProperties2Functions, {
    vkGetPhysicalDeviceProperties2KHR(
        physicalDevice: VkPhysicalDevice,
        pProperties: *mut VkPhysicalDeviceProperties2KHR
    ) -> (),

    vkGetPhysicalDeviceFeatures2KHR(
        physicalDevice: VkPhysicalDevice,
        pFeatures: *mut VkPhysicalDeviceFeatures2KHR
    ) -> (),

    vkGetPhysicalDeviceFormatProperties2KHR(
        physicalDevice: VkPhysicalDevice,
        pFormatProperties: *mut VkFormatProperties2KHR<'_>
    ) -> (),

    vkGetPhysicalDeviceImageFormatProperties2KHR(
        physicalDevice: VkPhysicalDevice,
        pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2KHR,
        pImageFormatProperties: *mut VkImageFormatProperties2KHR<'_>
    ) -> VkResult,

    vkGetPhysicalDeviceQueueFamilyProperties2KHR(
        physicalDevice: VkPhysicalDevice,
        pQueueFamilyPropertiesCount: *mut u32,
        pQueueFamilyProperties: *mut VkQueueFamilyProperties2KHR
    ) -> (),

    vkGetPhysicalDeviceMemoryProperties2KHR(
        physicalDevice: VkPhysicalDevice,
        pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2KHR
    ) -> (),

    vkGetPhysicalDeviceSparseImageFormatProperties2KHR(
        physicalDevice: VkPhysicalDevice,
        pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2KHR,
        pPropertyCount: *mut u32,
        pProperties: *mut VkSparseImageFormatProperties2KHR
    ) -> (),
});
