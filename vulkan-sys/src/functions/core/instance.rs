use crate::load_function_ptrs;
use crate::prelude::*;

use std::os::raw::c_char;
use std::os::raw::c_void;

load_function_ptrs!(InstanceFunctions, {
    vkGetDeviceProcAddr(device: VkDevice, pName: *const c_char) -> PFN_vkVoidFunction,

    vkDestroyInstance(instance: VkInstance, pAllocator: *const VkAllocationCallbacks) -> (),

    // physical device
    vkEnumeratePhysicalDevices(
        Instance: VkInstance,
        pPhysicalDeviceCount: *mut u32,
        pPhysicalDevices: *mut VkPhysicalDevice
    ) -> VkResult,

    vkGetPhysicalDeviceProperties(
        physicalDevice: VkPhysicalDevice,
        pProperties: *mut VkPhysicalDeviceProperties
    ) -> (),

    vkGetPhysicalDeviceFeatures(
        physicalDevice: VkPhysicalDevice,
        pFeatures: *mut VkPhysicalDeviceFeatures
    ) -> (),

    vkGetPhysicalDeviceFormatProperties(
        physicalDevice: VkPhysicalDevice,
        format: VkFormat,
        pFormatProperties: *mut VkFormatProperties
    ) -> (),

    vkGetPhysicalDeviceQueueFamilyProperties(
        physicalDevice: VkPhysicalDevice,
        pQueueFamilyPropertyCount: *mut u32,
        pQueueFamilyProperties: *mut VkQueueFamilyProperties
    ) -> (),

    vkGetPhysicalDeviceMemoryProperties(
        physicalDevice: VkPhysicalDevice,
        pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties
    ) -> (),

    vkGetPhysicalDeviceSparseImageFormatProperties(
        physicalDevice: VkPhysicalDevice,
        format: VkFormat,
        ty: VkImageType,
        samples: VkSampleCountFlags,
        usage: VkImageUsageFlagBits,
        tiling: VkImageTiling,
        pPropertyCount: *mut u32,
        pProperties: *mut VkSparseImageFormatProperties
    ) -> (),

    vkGetPhysicalDeviceImageFormatProperties(
        physicalDevice: VkPhysicalDevice,
        format: VkFormat,
        imageType: VkImageType,
        tiling: VkImageTiling,
        usage: VkImageUsageFlagBits,
        flags: VkImageCreateFlagBits,
        pImageFormatProperties: *mut VkImageFormatProperties
    ) -> VkResult,

    // device
    vkCreateDevice(
        physicalDevice: VkPhysicalDevice,
        pCreateInfo: *const VkDeviceCreateInfo<'_>,
        pAllocator: *const VkAllocationCallbacks,
        pDevice: *mut VkDevice
    ) -> VkResult,

    vkEnumerateDeviceExtensionProperties(
        physicalDevice: VkPhysicalDevice,
        pLayerName: *const c_char,
        pPropertyCount: *mut u32,
        pProperties: *mut VkExtensionProperties
    ) -> VkResult,
});
