use crate::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct VkSparseImageFormatProperties {
    pub aspectMask: VkImageAspectFlagBits,
    pub imageGranularity: VkExtent3D,
    pub flags: VkSparseImageFormatFlagBits,
}
