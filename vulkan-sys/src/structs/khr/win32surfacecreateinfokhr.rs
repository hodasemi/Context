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
pub struct VkWin32SurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkWin32SurfaceCreateFlagBitsKHR,
    pub hinstance: *mut c_void,
    pub hwnd: *mut c_void,
}

impl VkWin32SurfaceCreateInfoKHR {
    pub fn new<T, U, V>(flags: T, hinstance: &mut U, hwnd: &mut V) -> Self
    where
        T: Into<VkWin32SurfaceCreateFlagBitsKHR>,
    {
        VkWin32SurfaceCreateInfoKHR {
            sType: VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: flags.into(),
            hinstance: hinstance as *mut U as *mut c_void,
            hwnd: hwnd as *mut V as *mut c_void,
        }
    }
}
