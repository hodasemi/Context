pub use VkGeometryTypeNV::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkGeometryTypeNV {
    VK_GEOMETRY_TYPE_TRIANGLES_NV = 0,
    VK_GEOMETRY_TYPE_AABBS_NV = 1,
}
