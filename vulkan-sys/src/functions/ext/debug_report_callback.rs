use crate::load_function_ptrs;
use crate::prelude::*;

use std::os::raw::c_char;
use std::os::raw::c_void;

pub type PFN_vkDebugReportCallbackEXT = extern "system" fn(
    VkDebugReportFlagsEXT,
    VkDebugReportObjectTypeEXT,
    u64,
    usize,
    i32,
    *const c_char,
    *const c_char,
    *mut c_void,
) -> VkBool32;

load_function_ptrs!(DebugReportCallbackFunctions, {
    vkCreateDebugReportCallbackEXT(
        instance: VkInstance,
        createInfo: *const VkDebugReportCallbackCreateInfoEXT,
        allocationCallbacks: *const VkAllocationCallbacks,
        debugReportCallback: *mut VkDebugReportCallbackEXT
    ) -> VkResult,

    vkDestroyDebugReportCallbackEXT(
        instance: VkInstance,
        debugReportCallback: VkDebugReportCallbackEXT,
        allocationCallbacks: *const VkAllocationCallbacks
    ) -> (),
});
