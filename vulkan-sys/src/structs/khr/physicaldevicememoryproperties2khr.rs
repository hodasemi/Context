use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

pub type VkPhysicalDeviceMemoryProperties2 = VkPhysicalDeviceMemoryProperties2KHR;

#[repr(C)]
#[derive(Debug)]
pub struct VkPhysicalDeviceMemoryProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memoryProperties: VkPhysicalDeviceMemoryProperties,
}

impl VkPhysicalDeviceMemoryProperties2KHR {
    pub fn new(memory_properties: VkPhysicalDeviceMemoryProperties) -> Self {
        VkPhysicalDeviceMemoryProperties2KHR {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR,
            pNext: ptr::null(),
            memoryProperties: memory_properties,
        }
    }
}

impl Default for VkPhysicalDeviceMemoryProperties2KHR {
    fn default() -> Self {
        Self::new(VkPhysicalDeviceMemoryProperties::default())
    }
}

impl PNext<VkPhysicalDeviceMemoryBudgetPropertiesEXT> for VkPhysicalDeviceMemoryProperties2KHR {
    fn chain(&mut self, p_next: &VkPhysicalDeviceMemoryBudgetPropertiesEXT) {
        self.pNext = p_next as *const VkPhysicalDeviceMemoryBudgetPropertiesEXT as *const c_void;
    }
}
