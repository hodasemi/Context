pub use VkSubpassDescriptionFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkSubpassDescriptionFlags {
    VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX = 0x0000_0001,
    VK_SUBPASS_DESCRIPTION_PER_VIEW_POSITION_X_ONLY_BIT_NVX = 0x0000_0002,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkSubpassDescriptionFlagBits(u32);
SetupVkFlags!(VkSubpassDescriptionFlags, VkSubpassDescriptionFlagBits);
