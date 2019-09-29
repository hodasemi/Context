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
pub struct VkMemoryBarrier {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub srcAccessMask: VkAccessFlagBits,
    pub dstAccessMask: VkAccessFlagBits,
}

impl VkMemoryBarrier {
    pub fn new<S, T>(src_access_mask: S, dst_access_mask: T) -> VkMemoryBarrier
    where
        S: Into<VkAccessFlagBits>,
        T: Into<VkAccessFlagBits>,
    {
        VkMemoryBarrier {
            sType: VK_STRUCTURE_TYPE_MEMORY_BARRIER,
            pNext: ptr::null(),
            srcAccessMask: src_access_mask.into(),
            dstAccessMask: dst_access_mask.into(),
        }
    }
}
