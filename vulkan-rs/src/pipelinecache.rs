use utilities::prelude::*;

use crate::impl_vk_handle;
use crate::prelude::*;

use std::sync::Arc;

#[derive(Debug)]
pub struct PipelineCache {
    device: Arc<Device>,
    pipeline_cache: VkPipelineCache,
}

impl PipelineCache {
    pub fn new<T>(device: Arc<Device>, data: &T) -> VerboseResult<Arc<PipelineCache>> {
        let mut pipeline_cache_ci =
            VkPipelineCacheCreateInfo::new(VK_PIPELINE_CACHE_CREATE_NULL_BIT);

        pipeline_cache_ci.set_data(data);

        let pipeline_cache = device.create_pipeline_cache(&pipeline_cache_ci)?;

        Ok(Arc::new(PipelineCache {
            device,
            pipeline_cache,
        }))
    }

    pub fn get_data<T>(&self) -> VerboseResult<T> {
        self.device.pipeline_cache_data(self.pipeline_cache)
    }

    pub fn merge(&self, src_caches: &[&Arc<PipelineCache>]) -> VerboseResult<()> {
        let vk_caches: Vec<VkPipelineCache> = src_caches.iter().map(|c| c.vk_handle()).collect();

        self.device
            .merge_pipeline_cache(vk_caches.as_slice(), self.pipeline_cache)
    }
}

impl VulkanDevice for PipelineCache {
    fn device(&self) -> &Arc<Device> {
        &self.device
    }
}

impl_vk_handle!(PipelineCache, VkPipelineCache, pipeline_cache);

impl Drop for PipelineCache {
    fn drop(&mut self) {
        self.device.destroy_pipeline_cache(self.pipeline_cache);
    }
}
