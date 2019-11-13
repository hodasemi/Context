use crate::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct VkImageResolve {
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffset: VkOffset3D,
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffset: VkOffset3D,
    pub extent: VkExtent3D,
}
