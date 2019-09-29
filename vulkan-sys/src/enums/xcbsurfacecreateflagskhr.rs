pub use VkXcbSurfaceCreateFlagsKHR::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkXcbSurfaceCreateFlagsKHR {
    VK_XCB_SURFACE_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkXcbSurfaceCreateFlagBitsKHR(u32);
SetupVkFlags!(VkXcbSurfaceCreateFlagsKHR, VkXcbSurfaceCreateFlagBitsKHR);
