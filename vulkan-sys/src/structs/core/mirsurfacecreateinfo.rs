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
pub struct VkMirSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkMirSurfaceCreateFlagBitsKHR,
    pub connection: *mut c_void,
    pub mirSurface: *mut c_void,
}

impl VkMirSurfaceCreateInfoKHR {
    pub fn new<T, U, V>(flags: T, connection: &mut U, mir_surface: &mut V) -> Self
    where
        T: Into<VkMirSurfaceCreateFlagBitsKHR>,
    {
        VkMirSurfaceCreateInfoKHR {
            sType: VK_STRUCTURE_TYPE_MIR_SURFACE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: flags.into(),
            connection: connection as *mut U as *mut c_void,
            mirSurface: mir_surface as *mut V as *mut c_void,
        }
    }
}
