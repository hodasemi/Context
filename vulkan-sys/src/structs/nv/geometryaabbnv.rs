use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkGeometryAABBNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub aabbData: VkBuffer,
    pub numAABBs: u32,
    pub stride: u32,
    pub offset: VkDeviceSize,
}

impl VkGeometryAABBNV {
    pub fn new(aabb_data: VkBuffer, num_aabbs: u32, stride: u32, offset: VkDeviceSize) -> Self {
        VkGeometryAABBNV {
            sType: VK_STRUCTURE_TYPE_GEOMETRY_AABB_NV,
            pNext: ptr::null(),
            aabbData: aabb_data,
            numAABBs: num_aabbs,
            stride,
            offset,
        }
    }
}

impl Default for VkGeometryAABBNV {
    fn default() -> Self {
        Self::new(VkBuffer::default(), 0, 0, 0)
    }
}
