use crate::prelude::*;

use std::os::raw::c_void;

#[repr(C)]
#[derive(Debug)]
pub struct VkDisplayPresentInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcRect: VkRect2D,
    pub dstRect: VkRect2D,
    pub persistent: VkBool32,
}
