use crate::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct VkImageBlit {
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffsets: [VkOffset3D; 2],
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffsets: [VkOffset3D; 2],
}
