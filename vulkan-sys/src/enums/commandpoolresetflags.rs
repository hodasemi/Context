pub use VkCommandPoolResetFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkCommandPoolResetFlags {
    VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT = 0x00000001,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkCommandPoolResetFlagBits(u32);
SetupVkFlags!(VkCommandPoolResetFlags, VkCommandPoolResetFlagBits);
