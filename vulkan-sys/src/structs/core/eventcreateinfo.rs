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
pub struct VkEventCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkEventCreateFlagBits,
}

impl VkEventCreateInfo {
    pub fn new<T>(flags: T) -> Self
    where
        T: Into<VkEventCreateFlagBits>,
    {
        VkEventCreateInfo {
            sType: VK_STRUCTURE_TYPE_EVENT_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
        }
    }
}
