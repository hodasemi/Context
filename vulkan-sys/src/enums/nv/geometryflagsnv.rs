pub use VkGeometryFlagsNV::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkGeometryFlagsNV {
    VK_GEOMETRY_OPAQUE_BIT_NV = 0x0000_0001,
    VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_NV = 0x0000_0002,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkGeometryFlagBitsNV(u32);
SetupVkFlags!(VkGeometryFlagsNV, VkGeometryFlagBitsNV);
