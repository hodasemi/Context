pub use VkDependencyFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkDependencyFlags {
    VK_DEPENDENCY_BY_REGION_BIT = 0x000_00001,
    VK_DEPENDENCY_DEVICE_GROUP_BIT = 0x0000_0004,
    VK_DEPENDENCY_VIEW_LOCAL_BIT = 0x0000_0002,
    VK_DEPENDENCY_FLAG_BITS_MAX_ENUM = 0x7FFF_FFFF,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkDependencyFlagBits(u32);
SetupVkFlags!(VkDependencyFlags, VkDependencyFlagBits);
