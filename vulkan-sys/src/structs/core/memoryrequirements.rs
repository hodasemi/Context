use crate::prelude::*;

use std::cmp::Ordering;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkMemoryRequirements {
    pub size: VkDeviceSize,
    pub alignment: VkDeviceSize,
    pub memoryTypeBits: VkMemoryPropertyFlagBits,
}

impl Ord for VkMemoryRequirements {
    fn cmp(&self, other: &Self) -> Ordering {
        self.size.cmp(&other.size)
    }
}

impl PartialOrd for VkMemoryRequirements {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
