pub use VkRenderPassCreateFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkRenderPassCreateFlags {
    VK_RENDERPASS_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkRenderPassCreateFlagBits(u32);
SetupVkFlags!(VkRenderPassCreateFlags, VkRenderPassCreateFlagBits);
