pub use VkCommandPoolTrimFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkCommandPoolTrimFlags {
    VK_COMMAND_POOL_TRIM_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkCommandPoolTrimFlagBits(u32);
SetupVkFlags!(VkCommandPoolTrimFlags, VkCommandPoolTrimFlagBits);
