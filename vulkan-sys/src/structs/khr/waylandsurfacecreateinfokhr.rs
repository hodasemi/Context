use crate::prelude::*;

use super::super::{c_char_to_vkstring, raw_to_slice};

use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::os::raw::{c_char, c_double, c_ulong, c_void};
use std::ptr;
use std::slice;

#[repr(C)]
#[derive(Debug)]
pub struct VkWaylandSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkWaylandSurfaceCreateFlagBitsKHR,
    pub display: *mut c_void,
    pub surface: *mut c_void,
}

impl VkWaylandSurfaceCreateInfoKHR {
    pub fn new<T, U, V>(flags: T, display: &mut U, surface: &mut V) -> Self
    where
        T: Into<VkWaylandSurfaceCreateFlagBitsKHR>,
    {
        VkWaylandSurfaceCreateInfoKHR {
            sType: VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: flags.into(),
            display: display as *mut U as *mut c_void,
            surface: surface as *mut V as *mut c_void,
        }
    }
}
