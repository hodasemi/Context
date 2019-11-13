use crate::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct VkMemoryRequirements {
    pub size: VkDeviceSize,
    pub alignment: VkDeviceSize,
    pub memoryTypeBits: VkMemoryPropertyFlagBits,
}
