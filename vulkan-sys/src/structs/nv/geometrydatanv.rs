use crate::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct VkGeometryDataNV {
    pub triangles: VkGeometryTrianglesNV,
    pub aabbs: VkGeometryAABBNV,
}
