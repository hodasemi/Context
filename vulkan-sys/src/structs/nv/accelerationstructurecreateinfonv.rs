use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkAccelerationStructureCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub compactedSize: VkDeviceSize,
    pub info: VkAccelerationStructureInfoNV,
}

impl VkAccelerationStructureCreateInfoNV {
    pub fn new(compacted_size: VkDeviceSize, info: VkAccelerationStructureInfoNV) -> Self {
        VkAccelerationStructureCreateInfoNV {
            sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_NV,
            pNext: ptr::null(),
            compactedSize: compacted_size,
            info,
        }
    }
}
