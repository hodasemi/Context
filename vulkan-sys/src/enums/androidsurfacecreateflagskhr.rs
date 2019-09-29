pub use VkAndroidSurfaceCreateFlagsKHR::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkAndroidSurfaceCreateFlagsKHR {
    VK_ANDROID_SURFACE_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkAndroidSurfaceCreateFlagBitsKHR(u32);
SetupVkFlags!(
    VkAndroidSurfaceCreateFlagsKHR,
    VkAndroidSurfaceCreateFlagBitsKHR
);
