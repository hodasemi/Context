use utilities::prelude::*;

use crate::prelude::*;
use crate::{impl_vk_handle_t, mappedmemory::VkMappedMemory};

use std;
use std::mem;
use std::sync::Arc;

pub struct BufferBuilder<'a, T> {
    flags: VkBufferCreateFlagBits,
    usage: VkBufferUsageFlagBits,
    set_memory_properties: VkMemoryPropertyFlagBits,
    sharing_mode: VkSharingMode,
    data: Option<&'a [T]>,
    size: VkDeviceSize,
}

impl<'a, T> BufferBuilder<'a, T> {
    pub fn set_usage(mut self, usage: impl Into<VkBufferUsageFlagBits>) -> Self {
        self.usage = usage.into();

        self
    }

    pub fn set_memory_properties(
        mut self,
        set_memory_properties: impl Into<VkMemoryPropertyFlagBits>,
    ) -> Self {
        self.set_memory_properties = set_memory_properties.into();

        self
    }

    pub fn set_data(mut self, data: &'a [T]) -> Self {
        self.data = Some(data);

        self
    }

    pub fn set_size(mut self, size: VkDeviceSize) -> Self {
        self.size = size;

        self
    }

    pub fn set_sharing_mode(mut self, sharing_mode: VkSharingMode) -> Self {
        self.sharing_mode = sharing_mode;

        self
    }

    pub fn set_flags(mut self, flags: impl Into<VkBufferCreateFlagBits>) -> Self {
        self.flags = flags.into();

        self
    }
}

impl<'a, T: Copy> BufferBuilder<'a, T> {
    pub fn build(self, device: Arc<Device>) -> VerboseResult<Arc<Buffer<T>>> {
        let size = match self.data {
            Some(data) => data.len() as VkDeviceSize,
            None => self.size,
        };

        if size == 0 {
            create_error!("buffer size must not be zero");
        }

        // create buffer
        let buffer_ci = VkBufferCreateInfo::new(
            self.flags,
            size * mem::size_of::<T>() as VkDeviceSize,
            self.usage,
            self.sharing_mode,
            &[],
        );

        let buffer = device.create_buffer(&buffer_ci)?;

        // create memory
        let memory = Memory::buffer_memory(&device, self.set_memory_properties, buffer)?;

        let buffer = Arc::new(Buffer {
            device,
            buffer,
            memory,

            usage: self.usage,
            set_memory_properties: self.set_memory_properties,
            sharing_mode: self.sharing_mode,

            size,
        });

        if let Some(data) = self.data {
            buffer.fill(data)?;
        }

        Ok(buffer)
    }
}

#[derive(Debug)]
pub struct Buffer<T> {
    device: Arc<Device>,
    buffer: VkBuffer,
    memory: Arc<Memory<T>>,

    usage: VkBufferUsageFlagBits,
    set_memory_properties: VkMemoryPropertyFlagBits,
    sharing_mode: VkSharingMode,
    size: VkDeviceSize,
}

impl<T: Copy> Buffer<T> {
    pub fn fill(&self, data: &[T]) -> VerboseResult<()> {
        let mut buffer_map = self.map(data.len() as VkDeviceSize, 0)?;

        buffer_map.copy(data);

        Ok(())
    }

    pub fn map(
        &self,
        length: VkDeviceSize,
        offset: VkDeviceSize,
    ) -> VerboseResult<VkMappedMemory<'_, T>> {
        self.memory.map(length, offset)
    }

    pub fn map_complete(&self) -> VerboseResult<VkMappedMemory<'_, T>> {
        self.memory.map(self.size, 0)
    }
}

impl<T> Buffer<T> {
    pub fn new<'a>() -> BufferBuilder<'a, T> {
        BufferBuilder {
            flags: 0u32.into(),
            usage: 0u32.into(),
            set_memory_properties: 0u32.into(),
            sharing_mode: VK_SHARING_MODE_EXCLUSIVE,
            data: None,
            size: 0,
        }
    }

    pub fn byte_size(&self) -> VkDeviceSize {
        self.size * mem::size_of::<T>() as VkDeviceSize
    }

    pub fn size(&self) -> VkDeviceSize {
        self.size
    }
}

impl_vk_handle_t!(Buffer, VkBuffer, buffer);

impl<T> VkHandle<VkDeviceMemory> for Buffer<T> {
    fn vk_handle(&self) -> VkDeviceMemory {
        self.memory.vk_handle()
    }
}

impl<'a, T> VkHandle<VkDeviceMemory> for &'a Buffer<T> {
    fn vk_handle(&self) -> VkDeviceMemory {
        self.memory.vk_handle()
    }
}

impl<T> VkHandle<VkDeviceMemory> for Arc<Buffer<T>> {
    fn vk_handle(&self) -> VkDeviceMemory {
        self.memory.vk_handle()
    }
}

impl<'a, T> VkHandle<VkDeviceMemory> for &'a Arc<Buffer<T>> {
    fn vk_handle(&self) -> VkDeviceMemory {
        self.memory.vk_handle()
    }
}

impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        self.device.destroy_buffer(self.buffer);
    }
}
