use utilities::prelude::*;

use crate::impl_vk_handle;
use crate::prelude::*;

use std::mem;
use std::sync::Arc;

#[derive(Debug)]
pub struct QueryPool {
    device: Arc<Device>,
    query_pool: VkQueryPool,
    query_count: u32,
}

impl QueryPool {
    pub fn new(
        device: Arc<Device>,
        query_type: VkQueryType,
        query_count: u32,
        pipeline_statistics: impl Into<VkQueryPipelineStatisticFlagBits>,
    ) -> VerboseResult<Arc<QueryPool>> {
        let query_pool_ci = VkQueryPoolCreateInfo::new(
            VK_QUERY_POOL_CREATE_NULL_BIT,
            query_type,
            query_count,
            pipeline_statistics,
        );

        let query_pool = device.create_query_pool(&query_pool_ci)?;

        Ok(Arc::new(QueryPool {
            device,
            query_pool,
            query_count,
        }))
    }

    pub fn get_results(&self) -> VerboseResult<Vec<u64>> {
        let mut data = vec![0; self.query_count as usize];

        self.device.query_pool_results(
            self.query_pool,
            0,
            self.query_count,
            &mut data,
            mem::size_of::<u64>() as u64,
            VK_QUERY_RESULT_64_BIT,
        )?;

        Ok(data)
    }

    pub fn vk_query_pool(&self) -> VkQueryPool {
        self.query_pool
    }
}

impl_vk_handle!(QueryPool, VkQueryPool, query_pool);

impl Drop for QueryPool {
    fn drop(&mut self) {
        self.device.destroy_query_pool(self.query_pool);
    }
}
