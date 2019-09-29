use crate::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct VkClearAttachment {
    pub aspectMask: VkImageAspectFlagBits,
    pub colorAttachment: u32,
    pub clearValue: VkClearValue,
}
