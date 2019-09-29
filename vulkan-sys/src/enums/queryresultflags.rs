pub use VkQueryResultFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkQueryResultFlags {
    VK_QUERY_RESULT_64_BIT = 0x0000_0001,
    VK_QUERY_RESULT_WAIT_BIT = 0x0000_0002,
    VK_QUERY_RESULT_WITH_AVAILABILITY_BIT = 0x0000_0004,
    VK_QUERY_RESULT_PARTIAL_BIT = 0x0000_0008,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkQueryResultFlagBits(u32);
SetupVkFlags!(VkQueryResultFlags, VkQueryResultFlagBits);
