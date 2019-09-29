pub use VkIOSSurfaceCreateFlagsMVK::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkIOSSurfaceCreateFlagsMVK {
    VK_IOS_SURFACE_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkIOSSurfaceCreateFlagBitsMVK(u32);
SetupVkFlags!(VkIOSSurfaceCreateFlagsMVK, VkIOSSurfaceCreateFlagBitsMVK);
