pub use VkGeometryInstanceFlagsNV::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkGeometryInstanceFlagsNV {
    VK_GEOMETRY_INSTANCE_TRIANGLE_CULL_DISABLE_BIT_NV = 0x0000_0001,
    VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_NV = 0x0000_0002,
    VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_NV = 0x0000_0004,
    VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_NV = 0x0000_0008,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkGeometryInstanceFlagBitsNV(u32);
SetupVkFlags!(VkGeometryInstanceFlagsNV, VkGeometryInstanceFlagBitsNV);
