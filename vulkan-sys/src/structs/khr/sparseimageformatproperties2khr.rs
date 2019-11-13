use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

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
