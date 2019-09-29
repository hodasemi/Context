pub use VkImageUsageFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkImageUsageFlags {
    VK_IMAGE_USAGE_TRANSFER_SRC_BIT = 0x0000_0001,
    VK_IMAGE_USAGE_TRANSFER_DST_BIT = 0x0000_0002,
    VK_IMAGE_USAGE_SAMPLED_BIT = 0x0000_0004,
    VK_IMAGE_USAGE_STORAGE_BIT = 0x0000_0008,
    VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT = 0x0000_0010,
    VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT = 0x0000_0020,
    VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT = 0x0000_0040,
    VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT = 0x0000_0080,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash, Default)]
pub struct VkImageUsageFlagBits(u32);
SetupVkFlags!(VkImageUsageFlags, VkImageUsageFlagBits);
