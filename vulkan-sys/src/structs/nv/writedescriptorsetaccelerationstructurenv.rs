use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkWriteDescriptorSetAccelerationStructureNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub accelerationStructureCount: u32,
    pub pAccelerationStructures: *const VkAccelerationStructureNV,
}

impl VkWriteDescriptorSetAccelerationStructureNV {
    pub fn new(
        acceleration_structures: &[VkAccelerationStructureNV],
    ) -> VkWriteDescriptorSetAccelerationStructureNV {
        VkWriteDescriptorSetAccelerationStructureNV {
            sType: VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV,
            pNext: ptr::null(),
            accelerationStructureCount: acceleration_structures.len() as u32,
            pAccelerationStructures: acceleration_structures.as_ptr(),
        }
    }

    pub fn set_acceleration_structures<'a>(
        &'a mut self,
        acceleration_structures: &'a [VkAccelerationStructureNV],
    ) {
        self.accelerationStructureCount = acceleration_structures.len() as u32;
        self.pAccelerationStructures = acceleration_structures.as_ptr();
    }
}

impl Default for VkWriteDescriptorSetAccelerationStructureNV {
    fn default() -> Self {
        Self::new(&[])
    }
}
