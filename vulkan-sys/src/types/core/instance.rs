
use crate::prelude::*;
use crate::SetupUSizeConv;

use std::mem;
use std::ptr;

use std::os::raw::c_void;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkInstance(usize);
SetupUSizeConv!(VkInstance);

/*
impl VkInstance {
    pub fn create(instance_create_info: &VkInstanceCreateInfo) -> Result<Self, VkResult> {
        unsafe {
            let mut instance = mem::uninitialized();
            let result = vkCreateInstance(instance_create_info, ptr::null(), &mut instance);

            if result == VK_SUCCESS {
                Ok(instance)
            } else {
                Err(result)
            }
        }
    }

    pub fn create_with_allocation_callback(
        instance_create_info: &VkInstanceCreateInfo,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<Self, VkResult> {
        unsafe {
            let mut instance = mem::uninitialized();
            let result =
                vkCreateInstance(instance_create_info, allocation_callbacks, &mut instance);

            if result == VK_SUCCESS {
                Ok(instance)
            } else {
                Err(result)
            }
        }
    }

    pub fn enumerate_layer_properties() -> Result<Vec<VkLayerProperties>, VkResult> {
        let mut property_count: u32 = 0;

        // get the amount of properties
        let result =
            unsafe { vkEnumerateInstanceLayerProperties(&mut property_count, ptr::null_mut()) };

        if result != VK_SUCCESS {
            return Err(result);
        }

        let mut properties = Vec::with_capacity(property_count as usize);
        unsafe { properties.set_len(property_count as usize) };

        // get the properties
        let result = unsafe {
            vkEnumerateInstanceLayerProperties(&mut property_count, properties.as_mut_ptr())
        };

        if result == VK_SUCCESS {
            Ok(properties)
        } else {
            Err(result)
        }
    }

    pub fn enumerate_extension_properties(
        layer_name: Option<&VkString>,
    ) -> Result<Vec<VkExtensionProperties>, VkResult> {
        let mut count = 0;
        let name = match layer_name {
            Some(name) => name.as_ptr(),
            None => ptr::null(),
        };

        let mut result =
            unsafe { vkEnumerateInstanceExtensionProperties(name, &mut count, ptr::null_mut()) };

        if result != VK_SUCCESS {
            return Err(result);
        }

        let mut properties = Vec::with_capacity(count as usize);
        unsafe { properties.set_len(count as usize) };

        result = unsafe {
            vkEnumerateInstanceExtensionProperties(name, &mut count, properties.as_mut_ptr())
        };

        if result == VK_SUCCESS {
            Ok(properties)
        } else {
            Err(result)
        }
    }

    pub fn enumerate_physical_devices(&self) -> Result<Vec<VkPhysicalDevice>, VkResult> {
        let mut count = 0;

        let result = unsafe { vkEnumeratePhysicalDevices(*self, &mut count, ptr::null_mut()) };

        if result != VK_SUCCESS {
            return Err(result);
        }

        let mut physical_devices = Vec::with_capacity(count as usize);
        unsafe { physical_devices.set_len(count as usize) };

        let result =
            unsafe { vkEnumeratePhysicalDevices(*self, &mut count, physical_devices.as_mut_ptr()) };

        if result == VK_SUCCESS {
            Ok(physical_devices)
        } else {
            Err(result)
        }
    }

    pub fn get_proc_addr(&self, name: &VkString) -> Option<PFN_vkVoidFunction> {
        Some(unsafe { vkGetInstanceProcAddr(*self, name.as_ptr()) })
    }

    pub fn destroy(&self) {
        unsafe { vkDestroyInstance(*self, ptr::null()) };
    }

    pub fn destroy_with_allocation_callback(&self, allocation_callbacks: &VkAllocationCallbacks) {
        unsafe { vkDestroyInstance(*self, allocation_callbacks) };
    }
}
*/
