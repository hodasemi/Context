use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkGeometryNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub geometryType: VkGeometryTypeNV,
    pub geometry: VkGeometryDataNV,
    pub flags: VkGeometryFlagBitsNV,
}

impl VkGeometryNV {
    pub fn new<T>(geometry_type: VkGeometryTypeNV, geometry: VkGeometryDataNV, flags: T) -> Self
    where
        T: Into<VkGeometryFlagBitsNV>,
    {
        VkGeometryNV {
            sType: VK_STRUCTURE_TYPE_GEOMETRY_NV,
            pNext: ptr::null(),
            geometryType: geometry_type,
            geometry,
            flags: flags.into(),
        }
    }
}

unsafe impl Sync for VkGeometryNV {}
unsafe impl Send for VkGeometryNV {}
