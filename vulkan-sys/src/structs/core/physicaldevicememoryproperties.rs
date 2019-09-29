use crate::prelude::*;

use super::super::{c_char_to_vkstring, raw_to_slice};

use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::os::raw::{c_char, c_double, c_ulong, c_void};
use std::ptr;
use std::slice;

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
