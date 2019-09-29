pub use VkBuildAccelerationStructureFlagsNV::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkBuildAccelerationStructureFlagsNV {
    VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_NV = 0x0000_0001,
    VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_NV = 0x0000_0002,
    VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_NV = 0x0000_0004,
    VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_NV = 0x0000_0008,
    VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_NV = 0x0000_0010,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkBuildAccelerationStructureFlagBitsNV(u32);
SetupVkFlags!(
    VkBuildAccelerationStructureFlagsNV,
    VkBuildAccelerationStructureFlagBitsNV
);

impl Default for VkBuildAccelerationStructureFlagBitsNV {
    fn default() -> Self {
        VkBuildAccelerationStructureFlagBitsNV(0)
    }
}
