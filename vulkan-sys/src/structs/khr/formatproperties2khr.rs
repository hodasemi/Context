use crate::prelude::*;

use std::marker::PhantomData;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkFormatProperties2KHR<'a> {
    lt: PhantomData<&'a ()>,
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub formatProperties: VkFormatProperties,
}

impl<'a> VkFormatProperties2KHR<'a> {
    pub fn new(format_properties: VkFormatProperties) -> Self {
        VkFormatProperties2KHR {
            lt: PhantomData,
            sType: VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2_KHR,
            pNext: ptr::null(),
            formatProperties: format_properties,
        }
    }
}
