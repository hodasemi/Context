use vulkan_sys::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Block {
    memory: VkDeviceMemory,

    pub(crate) offset: VkDeviceSize,
    pub(crate) size: VkDeviceSize,

    pub(crate) used: bool,
}

impl Block {
    pub fn new(memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize) -> Self {
        Block {
            memory,
            offset,
            size,
            used: false,
        }
    }

    pub(crate) fn memory(&self) -> VkDeviceMemory {
        self.memory
    }
}
