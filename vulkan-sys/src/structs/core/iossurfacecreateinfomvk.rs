use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkIOSSurfaceCreateInfoMVK {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkIOSSurfaceCreateFlagBitsMVK,
    pub pView: *const c_void,
}

impl VkIOSSurfaceCreateInfoMVK {
    pub fn new<T, U>(flags: T, view: &U) -> Self
    where
        T: Into<VkIOSSurfaceCreateFlagBitsMVK>,
    {
        VkIOSSurfaceCreateInfoMVK {
            sType: VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK,
            pNext: ptr::null(),
            flags: flags.into(),
            pView: view as *const U as *const c_void,
        }
    }
}
