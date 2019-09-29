use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkAccelerationStructureMemoryRequirementsInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub r#type: VkAccelerationStructureMemoryRequirementsTypeNV,
    pub accelerationStructure: VkAccelerationStructureNV,
}

impl VkAccelerationStructureMemoryRequirementsInfoNV {
    pub fn new(
        r#type: VkAccelerationStructureMemoryRequirementsTypeNV,
        acceleration_structure: VkAccelerationStructureNV,
    ) -> Self {
        VkAccelerationStructureMemoryRequirementsInfoNV {
            sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV,
            pNext: ptr::null(),
            r#type,
            accelerationStructure: acceleration_structure,
        }
    }
}
