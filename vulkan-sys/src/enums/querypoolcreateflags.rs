pub use VkQueryPoolCreateFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkQueryPoolCreateFlags {
    VK_QUERY_POOL_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkQueryPoolCreateFlagBits(u32);
SetupVkFlags!(VkQueryPoolCreateFlags, VkQueryPoolCreateFlagBits);
