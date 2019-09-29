pub use VkQueryControlFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkQueryControlFlags {
    VK_QUERY_CONTROL_PRECISE_BIT = 0x0000_0001,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkQueryControlFlagBits(u32);
SetupVkFlags!(VkQueryControlFlags, VkQueryControlFlagBits);
