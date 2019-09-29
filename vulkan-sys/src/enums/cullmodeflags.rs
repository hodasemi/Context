pub use VkCullModeFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkCullModeFlags {
    VK_CULL_MODE_NONE = 0,
    VK_CULL_MODE_FRONT_BIT = 0x0000_0001,
    VK_CULL_MODE_BACK_BIT = 0x0000_0002,
    VK_CULL_MODE_FRONT_AND_BACK = 0x3,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkCullModeFlagBits(u32);
SetupVkFlags!(VkCullModeFlags, VkCullModeFlagBits);
