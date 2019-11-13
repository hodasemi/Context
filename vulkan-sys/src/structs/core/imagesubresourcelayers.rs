use crate::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct VkImageSubresourceLayers {
    pub aspectMask: VkImageAspectFlagBits,
    pub mipLevel: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}
