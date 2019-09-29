use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkGeometryDataNV {
    pub triangles: VkGeometryTrianglesNV,
    pub aabbs: VkGeometryAABBNV,
}
