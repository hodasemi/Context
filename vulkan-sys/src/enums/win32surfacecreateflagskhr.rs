pub use VkWin32SurfaceCreateFlagsKHR::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkWin32SurfaceCreateFlagsKHR {
    VK_WIN32_SURFACE_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkWin32SurfaceCreateFlagBitsKHR(u32);
SetupVkFlags!(
    VkWin32SurfaceCreateFlagsKHR,
    VkWin32SurfaceCreateFlagBitsKHR
);
