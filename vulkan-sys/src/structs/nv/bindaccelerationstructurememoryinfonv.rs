use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkBindAccelerationStructureMemoryInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub accelerationStructure: VkAccelerationStructureNV,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
    pub deviceIndexCount: u32,
    pub pDeviceIndices: *const u32,
}

impl VkBindAccelerationStructureMemoryInfoNV {
    pub fn new(
        acceleration_structure: VkAccelerationStructureNV,
        memory: VkDeviceMemory,
        memory_offset: VkDeviceSize,
        device_indices: &[u32],
    ) -> Self {
        VkBindAccelerationStructureMemoryInfoNV {
            sType: VK_STRUCTURE_TYPE_BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV,
            pNext: ptr::null(),
            accelerationStructure: acceleration_structure,
            memory,
            memoryOffset: memory_offset,
            deviceIndexCount: device_indices.len() as u32,
            pDeviceIndices: device_indices.as_ptr(),
        }
    }
}
