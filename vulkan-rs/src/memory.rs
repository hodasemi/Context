use crate::impl_vk_handle_t;
use crate::prelude::*;

use utilities::prelude::*;

use std::marker::PhantomData;
use std::sync::Arc;

#[derive(Debug)]
pub struct Memory<T> {
    device: Arc<Device>,

    memory: VkDeviceMemory,

    data_type: PhantomData<T>,
}

impl<T> Memory<T> {
    pub fn buffer_memory(
        device: &Arc<Device>,
        memory_properties: VkMemoryPropertyFlagBits,
        buffer: VkBuffer,
    ) -> VerboseResult<Arc<Memory<T>>> {
        let memory_requirements = device.buffer_memory_requirements(buffer);

        let memory = Self::new(device, memory_requirements, memory_properties)?;

        device.bind_buffer_memory(buffer, memory.vk_handle(), 0)?;

        Ok(memory)
    }

    pub fn image_memory(
        device: &Arc<Device>,
        memory_properties: VkMemoryPropertyFlagBits,
        image: VkImage,
    ) -> VerboseResult<Arc<Memory<T>>> {
        let memory_requirements = device.image_memory_requirements(image);

        let memory = Self::new(device, memory_requirements, memory_properties)?;

        device.bind_image_memory(image, memory.vk_handle(), 0)?;

        Ok(memory)
    }

    fn new(
        device: &Arc<Device>,
        memory_requirements: VkMemoryRequirements,
        memory_properties: VkMemoryPropertyFlagBits,
    ) -> VerboseResult<Arc<Memory<T>>> {
        let memory_type_index = device
            .memory_type_from_properties(memory_requirements.memoryTypeBits, memory_properties)?;

        let memory_ci = VkMemoryAllocateInfo::new(memory_requirements.size, memory_type_index);

        let memory = device.allocate_memory(&memory_ci)?;

        Ok(Arc::new(Memory {
            device: device.clone(),

            memory,

            data_type: PhantomData,
        }))
    }
}

impl<T: Copy> Memory<T> {
    pub fn map(
        &self,
        length: VkDeviceSize,
        offset: VkDeviceSize,
    ) -> VerboseResult<VkMappedMemory<'_, T>> {
        Ok(self
            .device
            .map_memory(self.memory, offset, length, VK_MEMORY_MAP_NULL_BIT)?)
    }
}

unsafe impl<T> Send for Memory<T> {}
unsafe impl<T> Sync for Memory<T> {}

impl_vk_handle_t!(Memory, VkDeviceMemory, memory);

impl<T> Drop for Memory<T> {
    fn drop(&mut self) {
        self.device.free_memory(self.memory);
    }
}
