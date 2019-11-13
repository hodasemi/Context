use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

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
