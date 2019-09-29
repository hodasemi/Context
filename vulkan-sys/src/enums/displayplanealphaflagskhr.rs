pub use VkDisplayPlaneAlphaFlagsKHR::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkDisplayPlaneAlphaFlagsKHR {
    VK_DISPLAY_PLANE_ALPHA_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkDisplayPlaneAlphaFlagBitsKHR(u32);
SetupVkFlags!(VkDisplayPlaneAlphaFlagsKHR, VkDisplayPlaneAlphaFlagBitsKHR);
