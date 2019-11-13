use crate::prelude::*;

use std::os::raw::{c_ulong, c_void};
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkXlibSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkXlibSurfaceCreateFlagBitsKHR,
    pub dpy: *mut c_void,
    pub window: c_ulong,
}

impl VkXlibSurfaceCreateInfoKHR {
    pub fn new<T, U>(flags: T, dpy: &mut U, window: c_ulong) -> Self
    where
        T: Into<VkXlibSurfaceCreateFlagBitsKHR>,
    {
        VkXlibSurfaceCreateInfoKHR {
            sType: VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: flags.into(),
            dpy: dpy as *mut U as *mut c_void,
            window: window,
        }
    }
}
