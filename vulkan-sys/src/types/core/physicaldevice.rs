use crate::prelude::*;
use crate::SetupUSizeConv;

use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkPhysicalDevice(usize);
SetupUSizeConv!(VkPhysicalDevice);

/*
impl VkPhysicalDevice {
    pub fn get_properties(&self) -> VkPhysicalDeviceProperties {
        unsafe {
            let mut physical_device_properties = mem::uninitialized();

            vkGetPhysicalDeviceProperties(*self, &mut physical_device_properties);

            physical_device_properties
        }
    }

    pub fn get_features(&self) -> VkPhysicalDeviceFeatures {
        unsafe {
            let mut physical_device_features = mem::uninitialized();

            vkGetPhysicalDeviceFeatures(*self, &mut physical_device_features);

            physical_device_features
        }
    }

    pub fn get_format_properties(&self, format: VkFormat) -> VkFormatProperties {
        unsafe {
            let mut physical_device_format_properties = mem::uninitialized();

            vkGetPhysicalDeviceFormatProperties(
                *self,
                format,
                &mut physical_device_format_properties,
            );

            physical_device_format_properties
        }
    }

    pub fn get_queue_family_properties(&self) -> Vec<VkQueueFamilyProperties> {
        let mut count = 0;

        unsafe {
            vkGetPhysicalDeviceQueueFamilyProperties(*self, &mut count, ptr::null_mut());
        }

        let mut queue_family_properties = Vec::with_capacity(count as usize);
        unsafe { queue_family_properties.set_len(count as usize) };

        unsafe {
            vkGetPhysicalDeviceQueueFamilyProperties(
                *self,
                &mut count,
                queue_family_properties.as_mut_ptr(),
            );
        }

        queue_family_properties
    }

    pub fn get_memory_properties(&self) -> VkPhysicalDeviceMemoryProperties {
        unsafe {
            let mut physical_device_memory_properties = mem::uninitialized();

            vkGetPhysicalDeviceMemoryProperties(*self, &mut physical_device_memory_properties);

            physical_device_memory_properties
        }
    }

    pub fn get_surface_support_khr(
        &self,
        surface: VkSurfaceKHR,
        queue_family_index: u32,
    ) -> Result<VkBool32, VkResult> {
        unsafe {
            let mut supported = mem::uninitialized();

            let result = vkGetPhysicalDeviceSurfaceSupportKHR(
                *self,
                queue_family_index,
                surface,
                &mut supported,
            );

            if result == VK_SUCCESS {
                Ok(supported)
            } else {
                Err(result)
            }
        }
    }

    pub fn get_surface_capabilities_khr(
        &self,
        surface: VkSurfaceKHR,
    ) -> Result<VkSurfaceCapabilitiesKHR, VkResult> {
        unsafe {
            let mut surface_capabilities = mem::uninitialized();

            let result = vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
                *self,
                surface,
                &mut surface_capabilities,
            );

            if result == VK_SUCCESS {
                Ok(surface_capabilities)
            } else {
                Err(result)
            }
        }
    }

    pub fn get_surface_formats_khr(
        &self,
        surface: VkSurfaceKHR,
    ) -> Result<Vec<VkSurfaceFormatKHR>, VkResult> {
        let mut count = 0;

        let result = unsafe {
            vkGetPhysicalDeviceSurfaceFormatsKHR(*self, surface, &mut count, ptr::null_mut())
        };

        if result != VK_SUCCESS {
            return Err(result);
        }

        let mut surface_formats = Vec::with_capacity(count as usize);
        unsafe { surface_formats.set_len(count as usize) };

        let result = unsafe {
            vkGetPhysicalDeviceSurfaceFormatsKHR(
                *self,
                surface,
                &mut count,
                surface_formats.as_mut_ptr(),
            )
        };

        if result == VK_SUCCESS {
            Ok(surface_formats)
        } else {
            Err(result)
        }
    }

    pub fn get_surface_present_modes_khr(
        &self,
        surface: VkSurfaceKHR,
    ) -> Result<Vec<VkPresentModeKHR>, VkResult> {
        let mut count = 0;

        let result = unsafe {
            vkGetPhysicalDeviceSurfacePresentModesKHR(*self, surface, &mut count, ptr::null_mut())
        };

        if result != VK_SUCCESS {
            return Err(result);
        }

        let mut surface_present_modes = Vec::with_capacity(count as usize);
        unsafe { surface_present_modes.set_len(count as usize) };

        let result = unsafe {
            vkGetPhysicalDeviceSurfacePresentModesKHR(
                *self,
                surface,
                &mut count,
                surface_present_modes.as_mut_ptr(),
            )
        };

        if result == VK_SUCCESS {
            Ok(surface_present_modes)
        } else {
            Err(result)
        }
    }

    /*
    pub fn get_features2_khr(&self) -> VkPhysicalDeviceFeatures2KHR {
        unsafe {
            let mut physical_device_features2_khr = mem::uninitialized();

            vkGetPhysicalDeviceFeatures2KHR(*self, &mut physical_device_features2_khr);

            physical_device_features2_khr
        }
    }
    */

    pub fn get_image_format_properties<T, U>(
        &self,
        format: VkFormat,
        image_type: VkImageType,
        tiling: VkImageTiling,
        usage: T,
        flags: U,
    ) -> Result<VkImageFormatProperties, VkResult>
    where
        T: Into<VkImageUsageFlagBits>,
        U: Into<VkImageCreateFlagBits>,
    {
        unsafe {
            let mut image_format_properties = mem::uninitialized();

            let result = vkGetPhysicalDeviceImageFormatProperties(
                *self,
                format,
                image_type,
                tiling,
                usage.into(),
                flags.into(),
                &mut image_format_properties,
            );

            if result == VK_SUCCESS {
                Ok(image_format_properties)
            } else {
                Err(result)
            }
        }
    }

    /*
    pub fn get_properties2_khr(&self) -> VkPhysicalDeviceProperties2KHR {
        unsafe {
            let mut physical_device_properties2_khr = mem::uninitialized();

            vkGetPhysicalDeviceProperties2KHR(*self, &mut physical_device_properties2_khr);

            physical_device_properties2_khr
        }
    }
    */

    pub fn enumerate_extension_properties(&self) -> Result<Vec<VkExtensionProperties>, VkResult> {
        let mut count = 0;

        let result = unsafe {
            vkEnumerateDeviceExtensionProperties(*self, ptr::null(), &mut count, ptr::null_mut())
        };

        if result != VK_SUCCESS {
            return Err(result);
        }

        let mut extension_properties = vec![VkExtensionProperties::empty(); count as usize];

        let result = unsafe {
            vkEnumerateDeviceExtensionProperties(
                *self,
                ptr::null(),
                &mut count,
                extension_properties.as_mut_ptr(),
            )
        };

        if result == VK_SUCCESS {
            Ok(extension_properties)
        } else {
            Err(result)
        }
    }
}
*/
