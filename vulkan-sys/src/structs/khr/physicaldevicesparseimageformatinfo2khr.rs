use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkPhysicalDeviceSparseImageFormatInfo2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub format: VkFormat,
    pub imageType: VkImageType,
    pub samples: VkSampleCountFlagBits,
    pub usage: VkImageUsageFlagBits,
    pub tiling: VkImageTiling,
}

impl VkPhysicalDeviceSparseImageFormatInfo2KHR {
    pub fn new<T, U>(
        format: VkFormat,
        image_type: VkImageType,
        samples: T,
        usage: U,
        tiling: VkImageTiling,
    ) -> Self
    where
        T: Into<VkSampleCountFlagBits>,
        U: Into<VkImageUsageFlagBits>,
    {
        VkPhysicalDeviceSparseImageFormatInfo2KHR {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR,
            pNext: ptr::null(),
            format,
            imageType: image_type,
            samples: samples.into(),
            usage: usage.into(),
            tiling,
        }
    }
}
