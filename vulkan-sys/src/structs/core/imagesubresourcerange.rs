use crate::prelude::*;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VkImageSubresourceRange {
    pub aspectMask: VkImageAspectFlagBits,
    pub baseMipLevel: u32,
    pub levelCount: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}
