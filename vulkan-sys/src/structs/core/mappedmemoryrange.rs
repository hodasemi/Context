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
pub struct VkMappedMemoryRange {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub memory: VkDeviceMemory,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
}

impl VkMappedMemoryRange {
    pub fn new(memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize) -> Self {
        VkMappedMemoryRange {
            sType: VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE,
            pNext: ptr::null(),
            memory,
            offset,
            size,
        }
    }
}
