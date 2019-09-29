use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

pub struct VkPhysicalDeviceMaintenance3PropertiesKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub maxPerSetDescriptors: u32,
    pub maxMemoryAllocationSize: VkDeviceSize,
}

impl VkPhysicalDeviceMaintenance3PropertiesKHR {
    pub fn new(max_per_set_descriptors: u32, max_memory_allocation_size: VkDeviceSize) -> Self {
        VkPhysicalDeviceMaintenance3PropertiesKHR {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES,
            pNext: ptr::null(),
            maxPerSetDescriptors: max_per_set_descriptors,
            maxMemoryAllocationSize: max_memory_allocation_size,
        }
    }
}
