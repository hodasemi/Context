use crate::load_function_ptrs;
use crate::prelude::*;

use std::os::raw::c_void;

pub type PFN_vkDebugUtilsMessengerCallbackEXT = extern "system" fn(
    VkDebugUtilsMessageSeverityFlagsEXT,
    VkDebugUtilsMessageTypeFlagsEXT,
    *const VkDebugUtilsMessengerCallbackDataEXT,
    *mut c_void,
) -> VkBool32;

load_function_ptrs!(DebugUtilsMessengerFunctions, {
    vkCreateDebugUtilsMessengerEXT(
        instance: VkInstance,
        createInfo: *const VkDebugUtilsMessengerCreateInfoEXT,
        allocationCallbacks: *const VkAllocationCallbacks,
        debugUtilsMessenger: *mut VkDebugUtilsMessengerEXT
    ) -> VkResult,

    vkDestroyDebugUtilsMessengerEXT(
        instance: VkInstance,
        debugUtilsMessenger: VkDebugUtilsMessengerEXT,
        allocationCallbacks: *const VkAllocationCallbacks
    ) -> (),
});
