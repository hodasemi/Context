use crate::prelude::*;

use std::os::raw::c_void;

#[repr(C)]
#[derive(Debug)]
pub struct VkDisplaySurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDisplaySurfaceCreateFlagBitsKHR,
    pub displayMode: VkDisplayModeKHR,
    pub planeIndex: u32,
    pub planeStackIndex: u32,
    pub transform: VkSurfaceTransformFlagBitsKHR,
    pub globalAlpha: f32,
    pub alphaMode: VkDisplayPlaneAlphaFlagBitsKHR,
    pub imageExtent: VkExtent2D,
}
