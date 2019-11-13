use crate::prelude::*;

#[repr(C)]
#[derive(Debug, Clone, Default, Copy)]
pub struct VkMemoryHeap {
    pub size: VkDeviceSize,
    pub flags: VkMemoryHeapFlagBits,
}
