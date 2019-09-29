pub use VkCommandBufferResetFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkCommandBufferResetFlags {
    VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT = 0x00000001,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkCommandBufferResetFlagBits(u32);
SetupVkFlags!(VkCommandBufferResetFlags, VkCommandBufferResetFlagBits);
