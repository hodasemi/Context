use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkQueueFamilyProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub queueFamilyProperties: VkQueueFamilyProperties,
}

impl VkQueueFamilyProperties2KHR {
    pub fn new(queue_family_properties: VkQueueFamilyProperties) -> Self {
        VkQueueFamilyProperties2KHR {
            sType: VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2_KHR,
            pNext: ptr::null(),
            queueFamilyProperties: queue_family_properties,
        }
    }
}
