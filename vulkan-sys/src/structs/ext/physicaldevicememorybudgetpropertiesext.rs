use crate::prelude::*;

use super::super::raw_to_slice;

use std::os::raw::c_void;
use std::ptr;

pub struct VkPhysicalDeviceMemoryBudgetPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub heapBudget: [VkDeviceSize; VK_MAX_MEMORY_HEAPS as usize],
    pub heapUsage: [VkDeviceSize; VK_MAX_MEMORY_HEAPS as usize],
}

impl VkPhysicalDeviceMemoryBudgetPropertiesEXT {
    pub fn new(
        heap_budget: [VkDeviceSize; VK_MAX_MEMORY_HEAPS as usize],
        heap_usage: [VkDeviceSize; VK_MAX_MEMORY_HEAPS as usize],
    ) -> Self {
        VkPhysicalDeviceMemoryBudgetPropertiesEXT {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT,
            pNext: ptr::null(),
            heapBudget: heap_budget,
            heapUsage: heap_usage,
        }
    }

    pub fn heap_budgets(&self, count: u32) -> &[VkDeviceSize] {
        raw_to_slice(self.heapBudget.as_ptr(), count)
    }

    pub fn heap_usages(&self, count: u32) -> &[VkDeviceSize] {
        raw_to_slice(self.heapUsage.as_ptr(), count)
    }
}

impl Default for VkPhysicalDeviceMemoryBudgetPropertiesEXT {
    fn default() -> Self {
        Self::new(
            [0; VK_MAX_MEMORY_HEAPS as usize],
            [0; VK_MAX_MEMORY_HEAPS as usize],
        )
    }
}
