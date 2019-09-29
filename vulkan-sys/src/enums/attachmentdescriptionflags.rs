pub use VkAttachmentDescriptionFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkAttachmentDescriptionFlags {
    VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT = 0x0000_0001,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkAttachmentDescriptionFlagBits(u32);
SetupVkFlags!(
    VkAttachmentDescriptionFlags,
    VkAttachmentDescriptionFlagBits
);
