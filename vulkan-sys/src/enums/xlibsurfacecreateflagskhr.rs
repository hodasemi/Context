pub use VkXlibSurfaceCreateFlagsKHR::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkXlibSurfaceCreateFlagsKHR {
    VK_XLIB_SURFACE_CREATE_NULL_BIT,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkXlibSurfaceCreateFlagBitsKHR(u32);
SetupVkFlags!(VkXlibSurfaceCreateFlagsKHR, VkXlibSurfaceCreateFlagBitsKHR);
