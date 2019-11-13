use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkImageCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkImageCreateFlagBits,
    pub imageType: VkImageType,
    pub format: VkFormat,
    pub extent: VkExtent3D,
    pub mipLevels: u32,
    pub arrayLayers: u32,
    pub samples: VkSampleCountFlagBits,
    pub tiling: VkImageTiling,
    pub usage: VkImageUsageFlagBits,
    pub sharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
    pub initialLayout: VkImageLayout,
}

impl VkImageCreateInfo {
    pub fn new<T, U, V>(
        flags: T,
        image_type: VkImageType,
        format: VkFormat,
        extent: VkExtent3D,
        mip_levels: u32,
        array_layers: u32,
        samples: U,
        tiling: VkImageTiling,
        usage: V,
        sharing_mode: VkSharingMode,
        queue_family_indices: &[u32],
        initial_layout: VkImageLayout,
    ) -> Self
    where
        T: Into<VkImageCreateFlagBits>,
        U: Into<VkSampleCountFlagBits>,
        V: Into<VkImageUsageFlagBits>,
    {
        VkImageCreateInfo {
            sType: VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            imageType: image_type,
            format,
            extent,
            mipLevels: mip_levels,
            arrayLayers: array_layers,
            samples: samples.into(),
            tiling,
            usage: usage.into(),
            sharingMode: sharing_mode,
            queueFamilyIndexCount: queue_family_indices.len() as u32,
            pQueueFamilyIndices: queue_family_indices.as_ptr(),
            initialLayout: initial_layout,
        }
    }
}
