pub use VkBufferViewCreateFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkBufferViewCreateFlags {
    VK_BUFFER_VIEW_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkBufferViewCreateFlagBits(u32);
SetupVkFlags!(VkBufferViewCreateFlags, VkBufferViewCreateFlagBits);
