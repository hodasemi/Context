use utilities::prelude::*;

use crate::impl_vk_handle;
use crate::prelude::*;

use std::sync::Arc;

pub struct DescriptorPoolBuilder {
    layout: Option<Arc<DescriptorSetLayout>>,
    descriptor_count: u32,
    flags: VkDescriptorPoolCreateFlagBits,
}

impl DescriptorPoolBuilder {
    pub fn set_flags(mut self, flags: impl Into<VkDescriptorPoolCreateFlagBits>) -> Self {
        self.flags |= flags.into();

        self
    }

    pub fn set_descriptor_set_count(mut self, count: u32) -> Self {
        self.descriptor_count = count;

        self
    }

    pub fn set_layout(mut self, layout: Arc<DescriptorSetLayout>) -> Self {
        self.layout = Some(layout);

        self
    }

    pub fn build(self, device: Arc<Device>) -> VerboseResult<Arc<DescriptorPool>> {
        if cfg!(debug_assertions) {
            if self.layout.is_none() {
                create_error!("no layout set!");
            }

            if self.descriptor_count == 0 {
                create_error!("descriptor count must be greater than 0");
            }
        }

        let layout = self.layout.ok_or("descriptor set layout was not set!")?;

        let descriptor_pool_ci =
            VkDescriptorPoolCreateInfo::new(self.flags, self.descriptor_count, layout.pool_sizes());

        let descriptor_pool = device.create_descriptor_pool(&descriptor_pool_ci)?;

        Ok(Arc::new(DescriptorPool {
            device,
            descriptor_pool,
            descriptor_set_layout: layout,
        }))
    }
}

#[derive(Debug)]
pub struct DescriptorPool {
    device: Arc<Device>,
    descriptor_pool: VkDescriptorPool,
    descriptor_set_layout: Arc<DescriptorSetLayout>,
}

impl DescriptorPool {
    pub fn builder() -> DescriptorPoolBuilder {
        DescriptorPoolBuilder {
            layout: None,
            descriptor_count: 1,
            flags: VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT.into(),
        }
    }

    pub fn reset(&self) -> VerboseResult<()> {
        self.device
            .reset_descriptor_pool(self.descriptor_pool, VK_DESCRIPTOR_POOL_RESET_NULL_BIT)
    }

    pub fn prepare_set(descriptor_pool: &Arc<DescriptorPool>) -> DescriptorSetBuilder {
        DescriptorSet::builder(descriptor_pool.device.clone(), descriptor_pool.clone())
    }
}

impl VulkanDevice for DescriptorPool {
    fn device(&self) -> &Arc<Device> {
        &self.device
    }
}

impl_vk_handle!(DescriptorPool, VkDescriptorPool, descriptor_pool);

impl VkHandle<VkDescriptorSetLayout> for DescriptorPool {
    fn vk_handle(&self) -> VkDescriptorSetLayout {
        self.descriptor_set_layout.vk_handle()
    }
}

impl<'a> VkHandle<VkDescriptorSetLayout> for &'a DescriptorPool {
    fn vk_handle(&self) -> VkDescriptorSetLayout {
        self.descriptor_set_layout.vk_handle()
    }
}

impl VkHandle<VkDescriptorSetLayout> for Arc<DescriptorPool> {
    fn vk_handle(&self) -> VkDescriptorSetLayout {
        self.descriptor_set_layout.vk_handle()
    }
}

impl<'a> VkHandle<VkDescriptorSetLayout> for &'a Arc<DescriptorPool> {
    fn vk_handle(&self) -> VkDescriptorSetLayout {
        self.descriptor_set_layout.vk_handle()
    }
}

impl Drop for DescriptorPool {
    fn drop(&mut self) {
        self.device.destroy_descriptor_pool(self.descriptor_pool);
    }
}

use crate::{ffi::*, handle_ffi_result};

#[no_mangle]
pub extern "C" fn create_descriptor_pool(
    flags: VkDescriptorPoolCreateFlagBits,
    descriptor_count: u32,
    descriptor_set_layout: *const DescriptorSetLayout,
    device: *const Device,
) -> *const DescriptorPool {
    let device = unsafe { Arc::from_raw(device) };
    let layout = unsafe { Arc::from_raw(descriptor_set_layout) };

    let pool_res = DescriptorPool::builder()
        .set_flags(flags)
        .set_descriptor_set_count(descriptor_count)
        .set_layout(layout)
        .build(device);

    handle_ffi_result!(pool_res)
}

#[no_mangle]
pub extern "C" fn reset_descriptor_pool(descriptor_pool: *const DescriptorPool) -> bool {
    let pool = unsafe { Arc::from_raw(descriptor_pool) };

    match pool.reset() {
        Ok(_) => true,
        Err(err) => {
            update_last_error(err);

            false
        }
    }
}

pub extern "C" fn destroy_descriptor_pool(descriptor_pool: *const DescriptorPool) {
    let _pool = unsafe { Arc::from_raw(descriptor_pool) };
}
