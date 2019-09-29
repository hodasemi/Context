use crate::prelude::*;
use crate::SetupU64Conv;

use std::mem;
use std::os::raw::{c_char, c_void};
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkDebugReportCallbackEXT(u64);
SetupU64Conv!(VkDebugReportCallbackEXT);

/*
impl VkDebugReportCallbackEXT {
    pub fn create(
        instance: VkInstance,
        debug_report_callback_create_info: &VkDebugReportCallbackCreateInfoEXT,
    ) -> Result<VkDebugReportCallbackEXT, VkResult> {
        INIT.call_once(|| Self::get_pfn(instance));

        match unsafe { vkCreateDebugReportCallbackEXT } {
            Some(create) => unsafe {
                let mut debug_report_callback = mem::uninitialized();

                let result = (create)(
                    instance,
                    debug_report_callback_create_info,
                    ptr::null(),
                    &mut debug_report_callback,
                );

                if result == VK_SUCCESS {
                    Ok(debug_report_callback)
                } else {
                    Err(result)
                }
            },
            None => Err(VK_ERROR_EXTENSION_NOT_PRESENT),
        }
    }

    pub fn create_with_allocation_callbacks(
        instance: VkInstance,
        debug_report_callback_create_info: &VkDebugReportCallbackCreateInfoEXT,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<VkDebugReportCallbackEXT, VkResult> {
        INIT.call_once(|| Self::get_pfn(instance));

        match unsafe { vkCreateDebugReportCallbackEXT } {
            Some(create) => unsafe {
                let mut debug_report_callback = mem::uninitialized();

                let result = (create)(
                    instance,
                    debug_report_callback_create_info,
                    allocation_callbacks,
                    &mut debug_report_callback,
                );

                if result == VK_SUCCESS {
                    Ok(debug_report_callback)
                } else {
                    Err(result)
                }
            },
            None => Err(VK_ERROR_EXTENSION_NOT_PRESENT),
        }
    }

    pub fn destroy(&self, instance: VkInstance) -> Result<(), VkResult> {
        INIT.call_once(|| Self::get_pfn(instance));

        match unsafe { vkDestroyDebugReportCallbackEXT } {
            Some(destroy) => {
                (destroy)(instance, *self, ptr::null());
                Ok(())
            }
            None => Err(VK_ERROR_EXTENSION_NOT_PRESENT),
        }
    }

    pub fn destroy_with_allocation_callbacks(
        &self,
        instance: VkInstance,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<(), VkResult> {
        INIT.call_once(|| Self::get_pfn(instance));

        match unsafe { vkDestroyDebugReportCallbackEXT } {
            Some(destroy) => {
                (destroy)(instance, *self, allocation_callbacks);
                Ok(())
            }
            None => Err(VK_ERROR_EXTENSION_NOT_PRESENT),
        }
    }

    fn get_pfn(instance: VkInstance) {
        unsafe {
            vkCreateDebugReportCallbackEXT = mem::transmute(
                instance.get_proc_addr(&VkString::new("vkCreateDebugReportCallbackEXT")),
            );

            vkDestroyDebugReportCallbackEXT = mem::transmute(
                instance.get_proc_addr(&VkString::new("vkDestroyDebugReportCallbackEXT")),
            );
        };
    }
}

*/
