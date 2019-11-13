use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkPhysicalDevicePushDescriptorPropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub maxPushDescriptors: u32,
}

impl VkPhysicalDevicePushDescriptorPropertiesKHR {
    pub fn new(max_push_descriptors: u32) -> Self {
        VkPhysicalDevicePushDescriptorPropertiesKHR {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR,
            pNext: ptr::null(),
            maxPushDescriptors: max_push_descriptors,
        }
    }
}
