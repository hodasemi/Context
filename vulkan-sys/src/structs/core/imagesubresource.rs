use crate::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct VkImageSubresource {
    pub aspectMask: VkImageAspectFlagBits,
    pub mipLevel: u32,
    pub arrayLayer: u32,
}
