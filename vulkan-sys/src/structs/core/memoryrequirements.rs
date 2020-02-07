use crate::prelude::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkMemoryRequirements {
    pub size: VkDeviceSize,
    pub alignment: VkDeviceSize,
    pub memoryTypeBits: VkMemoryPropertyFlagBits,
}
