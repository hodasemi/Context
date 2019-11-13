use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

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
