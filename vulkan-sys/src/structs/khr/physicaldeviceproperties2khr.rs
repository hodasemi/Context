use crate::impl_pnext;
use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
pub struct VkPhysicalDeviceProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub properties: VkPhysicalDeviceProperties,
}

impl VkPhysicalDeviceProperties2KHR {
    pub fn new(properties: VkPhysicalDeviceProperties) -> Self {
        VkPhysicalDeviceProperties2KHR {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2_KHR,
            pNext: ptr::null(),
            properties,
        }
    }
}

impl Default for VkPhysicalDeviceProperties2KHR {
    fn default() -> Self {
        Self::new(VkPhysicalDeviceProperties::default())
    }
}

impl_pnext!(
    VkPhysicalDeviceProperties2KHR,
    VkPhysicalDeviceRayTracingPropertiesNV
);

impl_pnext!(
    VkPhysicalDeviceProperties2KHR,
    VkPhysicalDeviceDescriptorIndexingPropertiesEXT
);
