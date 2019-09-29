pub use VkImageAspectFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkImageAspectFlags {
    VK_IMAGE_ASPECT_COLOR_BIT = 0x0000_0001,
    VK_IMAGE_ASPECT_DEPTH_BIT = 0x0000_0002,
    VK_IMAGE_ASPECT_STENCIL_BIT = 0x0000_0004,
    VK_IMAGE_ASPECT_METADATA_BIT = 0x0000_0008,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash, Default)]
pub struct VkImageAspectFlagBits(u32);
SetupVkFlags!(VkImageAspectFlags, VkImageAspectFlagBits);
