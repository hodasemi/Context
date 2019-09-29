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
pub struct VkPhysicalDeviceImageFormatInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub format: VkFormat,
    pub imageType: VkImageType,
    pub tiling: VkImageTiling,
    pub usage: VkImageUsageFlagBits,
    pub flags: VkImageCreateFlagBits,
}

impl VkPhysicalDeviceImageFormatInfo2KHR {
    pub fn new<T, U>(
        format: VkFormat,
        image_type: VkImageType,
        tiling: VkImageTiling,
        usage: T,
        flags: U,
    ) -> Self
    where
        T: Into<VkImageUsageFlagBits>,
        U: Into<VkImageCreateFlagBits>,
    {
        VkPhysicalDeviceImageFormatInfo2KHR {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR,
            pNext: ptr::null(),
            format,
            imageType: image_type,
            tiling,
            usage: usage.into(),
            flags: flags.into(),
        }
    }
}
