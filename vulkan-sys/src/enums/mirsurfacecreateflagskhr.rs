pub use VkMirSurfaceCreateFlagsKHR::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkMirSurfaceCreateFlagsKHR {
    VK_MIR_SURFACE_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkMirSurfaceCreateFlagBitsKHR(u32);
SetupVkFlags!(VkMirSurfaceCreateFlagsKHR, VkMirSurfaceCreateFlagBitsKHR);
