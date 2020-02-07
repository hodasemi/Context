use crate::prelude::*;
use utilities::prelude::*;

use super::chunk::Chunk;

use std::sync::Arc;

pub struct ChunkAllocator {
    size: VkDeviceSize,
}

impl ChunkAllocator {
    pub fn new(size: VkDeviceSize) -> Self {
        debug_assert!(Self::is_power_of_two(size));

        ChunkAllocator { size }
    }

    pub fn allocate(
        &self,
        device: Arc<Device>,
        size: VkDeviceSize,
        memory_type_index: u32,
    ) -> VerboseResult<Chunk> {
        let size = if size > self.size {
            Self::next_power_of_two(size)
        } else {
            self.size
        };

        Chunk::new(device, memory_type_index, size)
    }

    #[inline]
    fn is_power_of_two(size: VkDeviceSize) -> bool {
        let mut mask = 0;
        let power = (size as f64).log2() as VkDeviceSize;

        for i in 0..power {
            mask += 1 << i;
        }

        (size & mask) != mask
    }

    #[inline]
    fn next_power_of_two(size: VkDeviceSize) -> VkDeviceSize {
        let power = (size as f64).log2() as VkDeviceSize + 1;

        1 << power
    }
}
