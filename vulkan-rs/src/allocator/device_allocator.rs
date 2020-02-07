use crate::prelude::*;
use utilities::prelude::*;

use super::{block::Block, chunk::Chunk, chunk_allocator::ChunkAllocator};

use std::sync::Arc;

pub struct DeviceAllocator {
    chunk_allocator: ChunkAllocator,
    chunks: Vec<Chunk>,
}

impl DeviceAllocator {
    pub fn new(size: VkDeviceSize) -> Self {
        DeviceAllocator {
            chunk_allocator: ChunkAllocator::new(size),
            chunks: Vec::new(),
        }
    }

    pub fn allocate(
        &mut self,
        device: Arc<Device>,
        size: VkDeviceSize,
        memory_type_index: u32,
        alignment: VkDeviceSize,
    ) -> VerboseResult<Block> {
        for chunk in &mut self.chunks {
            if chunk.memory_type_index() == memory_type_index {
                if let Some(block) = chunk.allocate(size, alignment)? {
                    return Ok(block);
                }
            }
        }

        let mut new_chunk = self
            .chunk_allocator
            .allocate(device, size, memory_type_index)?;
        let block = new_chunk
            .allocate(size, alignment)?
            .ok_or("couldn't allocate memory")?;

        self.chunks.push(new_chunk);

        Ok(block)
    }

    pub fn deallocate(&mut self, block: &Block) {
        for chunk in &mut self.chunks {
            if chunk.contains(block) {
                chunk.deallocate(block);

                return;
            }
        }
    }
}
