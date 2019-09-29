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
pub struct VkXcbSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkXcbSurfaceCreateFlagBitsKHR,
    pub connection: *const c_void,
    pub window: u32,
}

impl VkXcbSurfaceCreateInfoKHR {
    pub fn new<T, U>(flags: T, connection: &mut U, window: u32) -> Self
    where
        T: Into<VkXcbSurfaceCreateFlagBitsKHR>,
    {
        VkXcbSurfaceCreateInfoKHR {
            sType: VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: flags.into(),
            connection: connection as *mut U as *mut c_void,
            window: window,
        }
    }
}
