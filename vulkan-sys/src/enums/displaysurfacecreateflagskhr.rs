pub use VkDisplaySurfaceCreateFlagsKHR::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkDisplaySurfaceCreateFlagsKHR {
    VK_DISPLAY_SURFACE_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkDisplaySurfaceCreateFlagBitsKHR(u32);
SetupVkFlags!(
    VkDisplaySurfaceCreateFlagsKHR,
    VkDisplaySurfaceCreateFlagBitsKHR
);
