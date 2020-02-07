use crate::allocator::block::Block;
use crate::prelude::*;

use utilities::prelude::*;

use std::marker::PhantomData;
use std::sync::Arc;

#[derive(Debug)]
pub struct Memory<T> {
    device: Arc<Device>,

    pub(crate) block: Block,

    data_type: PhantomData<T>,
}

impl<T> Memory<T> {
    pub(crate) fn forced_requirements(
        device: &Arc<Device>,
        memory_properties: VkMemoryPropertyFlagBits,
        buffer: VkBuffer,
        memory_requirements: VkMemoryRequirements,
    ) -> VerboseResult<Arc<Memory<T>>> {
        let memory = Self::new(device, memory_requirements, memory_properties)?;

        device.bind_buffer_memory(buffer, memory.block.memory(), memory.block.offset)?;

        Ok(memory)
    }

    pub(crate) fn buffer_memory(
        device: &Arc<Device>,
        memory_properties: VkMemoryPropertyFlagBits,
        buffer: VkBuffer,
    ) -> VerboseResult<Arc<Memory<T>>> {
        let memory_requirements = device.buffer_memory_requirements(buffer);

        Self::forced_requirements(device, memory_properties, buffer, memory_requirements)
    }

    pub(crate) fn image_memory(
        device: &Arc<Device>,
        memory_properties: VkMemoryPropertyFlagBits,
        image: VkImage,
    ) -> VerboseResult<Arc<Memory<T>>> {
        let memory_requirements = device.image_memory_requirements(image);

        let memory = Self::new(device, memory_requirements, memory_properties)?;

        device.bind_image_memory(image, memory.block.memory(), memory.block.offset)?;

        Ok(memory)
    }

    fn new(
        device: &Arc<Device>,
        memory_requirements: VkMemoryRequirements,
        memory_properties: VkMemoryPropertyFlagBits,
    ) -> VerboseResult<Arc<Memory<T>>> {
        let memory_type_index = device
            .memory_type_from_properties(memory_requirements.memoryTypeBits, memory_properties)?;

        let block = Device::allocate_memory_from_allocator(
            device,
            memory_requirements.size,
            memory_type_index,
            memory_requirements.alignment,
        )?;

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
        self.block.map(length)
    }
}

impl<T> Drop for Memory<T> {
    fn drop(&mut self) {
        self.device.free_memory_from_allocator(&self.block).unwrap();
    }
}
