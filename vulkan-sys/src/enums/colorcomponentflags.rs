pub use VkColorComponentFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkColorComponentFlags {
    VK_COLOR_COMPONENT_R_BIT = 0x0000_0001,
    VK_COLOR_COMPONENT_G_BIT = 0x0000_0002,
    VK_COLOR_COMPONENT_B_BIT = 0x0000_0004,
    VK_COLOR_COMPONENT_A_BIT = 0x0000_0008,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkColorComponentFlagBits(u32);
SetupVkFlags!(VkColorComponentFlags, VkColorComponentFlagBits);
