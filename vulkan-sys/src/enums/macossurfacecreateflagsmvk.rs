pub use VkMacOSSurfaceCreateFlagsMVK::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkMacOSSurfaceCreateFlagsMVK {
    VK_MACOS_SURFACE_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkMacOSSurfaceCreateFlagBitsMVK(u32);
SetupVkFlags!(
    VkMacOSSurfaceCreateFlagsMVK,
    VkMacOSSurfaceCreateFlagBitsMVK
);
