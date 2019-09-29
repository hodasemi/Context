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
pub struct VkImageViewCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkImageViewCreateFlagBits,
    pub image: VkImage,
    pub viewType: VkImageViewType,
    pub format: VkFormat,
    pub components: VkComponentMapping,
    pub subresourceRange: VkImageSubresourceRange,
}

impl VkImageViewCreateInfo {
    pub fn new<T>(
        flags: T,
        image: VkImage,
        view_type: VkImageViewType,
        format: VkFormat,
        components: VkComponentMapping,
        subresourceRange: VkImageSubresourceRange,
    ) -> Self
    where
        T: Into<VkImageViewCreateFlagBits>,
    {
        VkImageViewCreateInfo {
            sType: VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            image,
            viewType: view_type,
            format,
            components,
            subresourceRange,
        }
    }
}
