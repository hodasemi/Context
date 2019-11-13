use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkMacOSSurfaceCreateInfoMVK {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkMacOSSurfaceCreateFlagBitsMVK,
    pub pView: *const c_void,
}

impl VkMacOSSurfaceCreateInfoMVK {
    // TODO: replace 'U' with the actual type of a macos view
    pub fn new<T, U>(flags: T, view: &U) -> Self
    where
        T: Into<VkMacOSSurfaceCreateFlagBitsMVK>,
    {
        VkMacOSSurfaceCreateInfoMVK {
            sType: VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK,
            pNext: ptr::null(),
            flags: flags.into(),
            pView: view as *const U as *const c_void,
        }
    }
}
