use crate::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct VkImageFormatProperties {
    pub maxExtent: VkExtent3D,
    pub maxMipLevels: u32,
    pub maxArrayLayers: u32,
    pub sampleCounts: VkSampleCountFlagBits,
    pub maxResourceSize: VkDeviceSize,
}
