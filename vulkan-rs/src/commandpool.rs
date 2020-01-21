use utilities::prelude::*;

use crate::impl_vk_handle;
use crate::prelude::*;

use std::sync::Arc;

pub struct CommandPoolBuilder {
    flags: VkCommandPoolCreateFlagBits,
    queue_family_index: u32,
}

impl CommandPoolBuilder {
    pub fn set_flags(mut self, flags: impl Into<VkCommandPoolCreateFlagBits>) -> Self {
        self.flags = flags.into();

        self
    }

    pub fn set_queue_family_index(mut self, queue_family_index: u32) -> Self {
        self.queue_family_index = queue_family_index;

        self
    }

    pub fn build(self, device: Arc<Device>) -> VerboseResult<Arc<CommandPool>> {
        let command_pool_ci = VkCommandPoolCreateInfo::new(self.flags, self.queue_family_index);

        let command_pool = device.create_command_pool(&command_pool_ci)?;

        Ok(Arc::new(CommandPool {
            device,
            command_pool,
        }))
    }
}

#[derive(Debug)]
pub struct CommandPool {
    device: Arc<Device>,
    command_pool: VkCommandPool,
}

impl CommandPool {
    pub fn builder() -> CommandPoolBuilder {
        CommandPoolBuilder {
            flags: VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT
                | VK_COMMAND_POOL_CREATE_TRANSIENT_BIT,
            queue_family_index: 0,
        }
    }

    pub fn allocate_primary_buffer(pool: &Arc<CommandPool>) -> VerboseResult<Arc<CommandBuffer>> {
        CommandBuffer::primary().build(pool.device.clone(), pool)
    }

    pub fn allocate_secondary_buffer(pool: &Arc<CommandPool>) -> VerboseResult<Arc<CommandBuffer>> {
        CommandBuffer::secondary().build(pool.device.clone(), pool)
    }
}

impl VulkanDevice for CommandPool {
    fn device(&self) -> &Arc<Device> {
        &self.device
    }
}

impl_vk_handle!(CommandPool, VkCommandPool, command_pool);

impl Drop for CommandPool {
    fn drop(&mut self) {
        self.device.destroy_command_pool(self.command_pool);
    }
}
