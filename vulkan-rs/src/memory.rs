use crate::allocator::block::Block;
use crate::prelude::*;

use utilities::prelude::*;

use std::marker::PhantomData;
use std::sync::Arc;

#[derive(Debug)]
pub struct Memory<T> {
    device: Arc<Device>,

    block: Block,

    data_type: PhantomData<T>,
}

impl<T> Memory<T> {
    pub(crate) fn buffer_memory(
        device: &Arc<Device>,
        memory_properties: VkMemoryPropertyFlagBits,
        buffer: VkBuffer,
    ) -> VerboseResult<Arc<Memory<T>>> {
        let memory_requirements = device.buffer_memory_requirements(buffer);

        let memory = Self::new(device, memory_requirements, memory_properties)?;

        device.bind_buffer_memory(buffer, memory.block.memory(), 0)?;

        Ok(memory)
    }

    pub(crate) fn image_memory(
        device: &Arc<Device>,
        memory_properties: VkMemoryPropertyFlagBits,
        image: VkImage,
    ) -> VerboseResult<Arc<Memory<T>>> {
        let memory_requirements = device.image_memory_requirements(image);

        let memory = Self::new(device, memory_requirements, memory_properties)?;

        device.bind_image_memory(image, memory.block.memory(), 0)?;

        Ok(memory)
    }

    fn new(
        device: &Arc<Device>,
        memory_requirements: VkMemoryRequirements,
        memory_properties: VkMemoryPropertyFlagBits,
    ) -> VerboseResult<Arc<Memory<T>>> {
        let memory_type_index = device
            .memory_type_from_properties(memory_requirements.memoryTypeBits, memory_properties)?;

        let block =
            device.allocate_memory_from_allocator(memory_requirements.size, memory_type_index)?;

        Ok(Arc::new(Memory {
            device: device.clone(),

            block,

            data_type: PhantomData,
        }))
    }

    pub(crate) fn vk_handle(&self) -> VkDeviceMemory {
        self.block.memory()
    }
}

impl<T> VulkanDevice for Memory<T> {
    fn device(&self) -> &Arc<Device> {
        &self.device
    }
}

impl<T: Clone> Memory<T> {
    pub fn map(&self, length: VkDeviceSize) -> VerboseResult<VkMappedMemory<'_, T>> {
        debug_assert!(length <= length * std::mem::size_of::<T>() as VkDeviceSize);

        Ok(self.device.map_memory(
            self.block.memory(),
            self.block.offset,
            length,
            VK_MEMORY_MAP_NULL_BIT,
        )?)
    }
}

impl<T> Drop for Memory<T> {
    fn drop(&mut self) {
        self.device.free_memory_from_allocator(&self.block).unwrap();
    }
}
