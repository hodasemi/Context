use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VkAccelerationStructureInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub r#type: VkAccelerationStructureTypeNV,
    pub flags: VkBuildAccelerationStructureFlagBitsNV,
    pub instanceCount: u32,
    pub geometryCount: u32,
    pub pGeometries: *const VkGeometryNV,
}

impl VkAccelerationStructureInfoNV {
    pub fn bottom_level(
        flags: impl Into<VkBuildAccelerationStructureFlagBitsNV>,
        instance_count: u32,
        geometries: &[VkGeometryNV],
    ) -> Self {
        VkAccelerationStructureInfoNV {
            sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_INFO_NV,
            pNext: ptr::null(),
            r#type: VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_NV,
            flags: flags.into(),
            instanceCount: instance_count,
            geometryCount: geometries.len() as u32,
            pGeometries: geometries.as_ptr(),
        }
    }

    pub fn top_level(
        flags: impl Into<VkBuildAccelerationStructureFlagBitsNV>,
        instance_count: u32,
        geometries: &[VkGeometryNV],
    ) -> Self {
        VkAccelerationStructureInfoNV {
            sType: VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_INFO_NV,
            pNext: ptr::null(),
            r#type: VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_NV,
            flags: flags.into(),
            instanceCount: instance_count,
            geometryCount: geometries.len() as u32,
            pGeometries: geometries.as_ptr(),
        }
    }
}
