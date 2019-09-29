pub use VkFenceCreateFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkFenceCreateFlags {
    VK_FENCE_CREATE_SIGNALED_BIT = 0x0000_0001,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkFenceCreateFlagBits(u32);
SetupVkFlags!(VkFenceCreateFlags, VkFenceCreateFlagBits);
