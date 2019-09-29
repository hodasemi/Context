use crate::prelude::*;

use std::marker::PhantomData;
use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkAndroidSurfaceCreateInfoKHR<'a> {
    lt: PhantomData<&'a ()>,
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkAndroidSurfaceCreateFlagBitsKHR,
    pub window: *mut c_void,
}

impl<'a> VkAndroidSurfaceCreateInfoKHR<'a> {
    pub fn new<T, U>(flags: T, window: &mut U) -> Self
    where
        T: Into<VkAndroidSurfaceCreateFlagBitsKHR>,
    {
        VkAndroidSurfaceCreateInfoKHR {
            lt: PhantomData,
            sType: VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: flags.into(),
            window: window as *mut U as *mut c_void,
        }
    }
}
