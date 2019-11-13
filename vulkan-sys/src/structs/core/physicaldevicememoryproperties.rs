use crate::prelude::*;

use super::super::raw_to_slice;

#[repr(C)]
#[derive(Debug)]
pub struct VkPhysicalDeviceMemoryProperties {
    pub memoryTypeCount: u32,
    pub memoryTypes: [VkMemoryType; VK_MAX_MEMORY_TYPES as usize],
    pub memoryHeapCount: u32,
    pub memoryHeaps: [VkMemoryHeap; VK_MAX_MEMORY_HEAPS as usize],
}

impl VkPhysicalDeviceMemoryProperties {
    pub fn memory_types(&self) -> &[VkMemoryType] {
        raw_to_slice(self.memoryTypes.as_ptr(), self.memoryTypeCount)
    }

    pub fn memory_heaps(&self) -> &[VkMemoryHeap] {
        raw_to_slice(self.memoryHeaps.as_ptr(), self.memoryHeapCount)
    }
}

impl Default for VkPhysicalDeviceMemoryProperties {
    fn default() -> Self {
        VkPhysicalDeviceMemoryProperties {
            memoryTypeCount: 0,
            memoryTypes: [VkMemoryType::default(); VK_MAX_MEMORY_TYPES as usize],
            memoryHeapCount: 0,
            memoryHeaps: [VkMemoryHeap::default(); VK_MAX_MEMORY_HEAPS as usize],
        }
    }
}
