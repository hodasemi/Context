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
pub struct VkSparseImageFormatProperties2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub properties: VkSparseImageFormatProperties,
}

impl VkSparseImageFormatProperties2KHR {
    pub fn new(properties: VkSparseImageFormatProperties) -> Self {
        VkSparseImageFormatProperties2KHR {
            sType: VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR,
            pNext: ptr::null(),
            properties,
        }
    }
}
