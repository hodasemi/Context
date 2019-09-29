pub use VkImageViewCreateFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkImageViewCreateFlags {
    VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT = 0x0000_0001,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkImageViewCreateFlagBits(u32);
SetupVkFlags!(VkImageViewCreateFlags, VkImageViewCreateFlagBits);
