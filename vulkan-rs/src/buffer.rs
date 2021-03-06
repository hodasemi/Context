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

    forced_requirements: Option<VkMemoryRequirements>,
}

impl<'a, T> BufferBuilder<'a, T> {
    pub fn set_usage(mut self, usage: impl Into<VkBufferUsageFlagBits>) -> Self {
        self.usage = usage.into();

        self
    }

    pub(crate) fn force_requirements(mut self, memory_requirements: VkMemoryRequirements) -> Self {
        self.forced_requirements = Some(memory_requirements);

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

impl<'a, T: Clone> BufferBuilder<'a, T> {
    pub fn build(self, device: Arc<Device>) -> VerboseResult<Arc<Buffer<T>>> {
        let size = match self.data {
            Some(data) => data.len() as VkDeviceSize,
            None => match &self.forced_requirements {
                Some(memory_requirements) => memory_requirements.size,
                None => self.size,
            },
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
        let memory = match self.forced_requirements {
            Some(memory_requirements) => Memory::forced_requirements(
                &device,
                self.set_memory_properties,
                buffer,
                memory_requirements,
            )?,
            None => Memory::buffer_memory(&device, self.set_memory_properties, buffer)?,
        };

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

impl<T: Clone> Buffer<T> {
    pub fn fill(&self, data: &[T]) -> VerboseResult<()> {
        let mut buffer_map = self.map(data.len() as VkDeviceSize)?;

        buffer_map.copy(data);

        Ok(())
    }

    pub fn map(&self, length: VkDeviceSize) -> VerboseResult<VkMappedMemory<'_, T>> {
        self.memory.map(length)
    }

    pub fn map_complete(&self) -> VerboseResult<VkMappedMemory<'_, T>> {
        self.memory.map(self.size)
    }

    pub fn into_device_local(
        &self,
        command_buffer: &Arc<CommandBuffer>,
        access_mask: impl Into<VkAccessFlagBits>,
        stage: impl Into<VkPipelineStageFlagBits>,
    ) -> VerboseResult<Arc<Buffer<T>>> {
        let new_usage =
            (self.usage ^ VK_BUFFER_USAGE_TRANSFER_SRC_BIT) | VK_BUFFER_USAGE_TRANSFER_DST_BIT;

        let device_local_buffer = Buffer::builder()
            .set_memory_properties(VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT)
            .set_usage(new_usage)
            .set_size(self.size)
            .build(self.device.clone())?;

        // copy complete buffer
        command_buffer.copy_buffer(
            &self,
            &device_local_buffer,
            &[VkBufferCopy {
                srcOffset: 0,
                dstOffset: 0,
                size: self.byte_size(),
            }],
        );

        // make sure buffer is copied before using it
        command_buffer.buffer_barrier(
            &device_local_buffer,
            VK_ACCESS_TRANSFER_WRITE_BIT,
            VK_PIPELINE_STAGE_TRANSFER_BIT,
            access_mask,
            stage,
        );

        Ok(device_local_buffer)
    }
}

impl<T> Buffer<T> {
    pub fn builder<'a>() -> BufferBuilder<'a, T> {
        BufferBuilder {
            flags: 0u32.into(),
            usage: 0u32.into(),
            set_memory_properties: 0u32.into(),
            sharing_mode: VK_SHARING_MODE_EXCLUSIVE,
            data: None,
            size: 0,

            forced_requirements: None,
        }
    }

    pub(crate) fn offset(&self) -> VkDeviceSize {
        self.memory.block.offset
    }

    pub(crate) fn byte_size(&self) -> VkDeviceSize {
        self.size * mem::size_of::<T>() as VkDeviceSize
    }

    pub fn size(&self) -> VkDeviceSize {
        self.size
    }
}

impl<T> VulkanDevice for Buffer<T> {
    fn device(&self) -> &Arc<Device> {
        &self.device
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

// use crate::{ffi::*, handle_ffi_result};

// pub trait FFIBuffer {}

// #[no_mangle]
// pub extern "C" fn create_buffer(device: *const Device) -> *const dyn FFIBuffer {
//     todo!()
// }
