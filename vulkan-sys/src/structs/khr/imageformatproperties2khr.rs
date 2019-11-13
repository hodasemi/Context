use crate::prelude::*;

use std::marker::PhantomData;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkImageFormatProperties2KHR<'a> {
    lt: PhantomData<&'a ()>,
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub imageFormatProperties: VkImageFormatProperties,
}

impl<'a> VkImageFormatProperties2KHR<'a> {
    pub fn new(image_format_properties: VkImageFormatProperties) -> Self {
        VkImageFormatProperties2KHR {
            lt: PhantomData,
            sType: VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2_KHR,
            pNext: ptr::null(),
            imageFormatProperties: image_format_properties,
        }
    }
}
