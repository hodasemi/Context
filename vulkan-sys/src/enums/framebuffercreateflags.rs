pub use VkFramebufferCreateFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkFramebufferCreateFlags {
    VK_FRAMEBUFFER_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkFramebufferCreateFlagBits(u32);
SetupVkFlags!(VkFramebufferCreateFlags, VkFramebufferCreateFlagBits);
