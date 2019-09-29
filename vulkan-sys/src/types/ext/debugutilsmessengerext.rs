use crate::prelude::*;
use crate::SetupU64Conv;

use std::mem;
use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkDebugUtilsMessengerEXT(u64);
SetupU64Conv!(VkDebugUtilsMessengerEXT);

/*
impl VkDebugUtilsMessengerEXT {
    pub fn create(
        instance: VkInstance,
        debug_utils_messenger_create_info: &VkDebugUtilsMessengerCreateInfoEXT,
    ) -> Result<VkDebugUtilsMessengerEXT, VkResult> {
        INIT.call_once(|| Self::get_pfn(instance));

        match unsafe { vkCreateDebugUtilsMessengerEXT } {
            Some(create) => unsafe {
                let mut debug_utils_messenger = mem::uninitialized();

                let result = (create)(
                    instance,
                    debug_utils_messenger_create_info,
                    ptr::null(),
                    &mut debug_utils_messenger,
                );

                if result == VK_SUCCESS {
                    Ok(debug_utils_messenger)
                } else {
                    Err(result)
                }
            },
            None => Err(VK_ERROR_EXTENSION_NOT_PRESENT),
        }
    }

    pub fn create_with_allocation_callbacks(
        instance: VkInstance,
        debug_utils_messenger_create_info: &VkDebugUtilsMessengerCreateInfoEXT,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<VkDebugUtilsMessengerEXT, VkResult> {
        INIT.call_once(|| Self::get_pfn(instance));

        match unsafe { vkCreateDebugUtilsMessengerEXT } {
            Some(create) => unsafe {
                let mut debug_utils_messenger = mem::uninitialized();

                let result = (create)(
                    instance,
                    debug_utils_messenger_create_info,
                    allocation_callbacks,
                    &mut debug_utils_messenger,
                );

                if result == VK_SUCCESS {
                    Ok(debug_utils_messenger)
                } else {
                    Err(result)
                }
            },
            None => Err(VK_ERROR_EXTENSION_NOT_PRESENT),
        }
    }

    pub fn destroy(&self, instance: VkInstance) -> Result<(), VkResult> {
        INIT.call_once(|| Self::get_pfn(instance));

        match unsafe { vkDestroyDebugUtilsMessengerEXT } {
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

        match unsafe { vkDestroyDebugUtilsMessengerEXT } {
            Some(destroy) => {
                (destroy)(instance, *self, allocation_callbacks);
                Ok(())
            }
            None => Err(VK_ERROR_EXTENSION_NOT_PRESENT),
        }
    }

    pub fn get_pfn(instance: VkInstance) {
        unsafe {
            vkCreateDebugUtilsMessengerEXT = mem::transmute(
                instance.get_proc_addr(&VkString::new("vkCreateDebugUtilsMessengerEXT")),
            );

            vkDestroyDebugUtilsMessengerEXT = mem::transmute(
                instance.get_proc_addr(&VkString::new("vkDestroyDebugUtilsMessengerEXT")),
            );
        };
    }
}
*/
