pub use VkWaylandSurfaceCreateFlagsKHR::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkWaylandSurfaceCreateFlagsKHR {
    VK_WAYLAND_SURFACE_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkWaylandSurfaceCreateFlagBitsKHR(u32);
SetupVkFlags!(
    VkWaylandSurfaceCreateFlagsKHR,
    VkWaylandSurfaceCreateFlagBitsKHR
);
