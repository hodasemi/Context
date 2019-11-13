use crate::prelude::*;

#[repr(C)]
#[derive(Debug, Clone, Default, Copy)]
pub struct VkMemoryType {
    pub propertyFlagBits: VkMemoryPropertyFlagBits,
    pub heapIndex: u32,
}
