use crate::load_function_ptrs;
use crate::prelude::*;

use std::os::raw::c_char;
use std::os::raw::c_void;

pub type PFN_vkEnumerateInstanceExtensionProperties = extern "system" fn(
    pLayerName: *const c_char,
    pPropertyCount: *mut u32,
    pProperties: *mut VkExtensionProperties,
) -> VkResult;

pub type PFN_vkEnumerateInstanceLayerProperties =
    extern "system" fn(pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult;

load_function_ptrs!(EntryFunctions, {
   vkCreateInstance(
        pCreateInfo: *const VkInstanceCreateInfo<'_>,
        pAllocator: *const VkAllocationCallbacks,
        pInstance: *mut VkInstance
    ) -> VkResult,

    vkEnumerateInstanceExtensionProperties(
        pLayerName: *const c_char,
        pPropertyCount: *mut u32,
        pProperties: *mut VkExtensionProperties
    ) -> VkResult,

    vkEnumerateInstanceLayerProperties(
        pPropertyCount: *mut u32,
        pProperties: *mut VkLayerProperties
    ) -> VkResult,
});
