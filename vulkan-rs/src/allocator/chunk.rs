use crate::prelude::*;
use utilities::prelude::*;

use super::block::Block;

pub struct Chunk {
    memory: VkDeviceMemory,
    memory_type_index: u32,
    size: VkDeviceSize,

    blocks: Vec<Block>,
}

impl Chunk {
    pub fn new(device: &Device, memory_type_index: u32, size: VkDeviceSize) -> VerboseResult<Self> {
        let memory_ci = VkMemoryAllocateInfo::new(size, memory_type_index);

        let memory = device.allocate_memory(&memory_ci)?;

        Ok(Chunk {
            memory,
            memory_type_index,
            size,

            blocks: vec![Block::new(memory, 0, size)],
        })
    }

    pub fn allocate(
        &mut self,
        size: VkDeviceSize,
        alignment: VkDeviceSize,
    ) -> VerboseResult<Option<Block>> {
        if self.size < size {
            create_error!("chunk is too small for requested size");
        }

        let mut result_block = None;
        let mut new_block = None;

        for block in &mut self.blocks {
            if !block.used {
                let mut new_size = block.size;

                if block.offset % alignment != 0 {
                    new_size -= alignment - block.offset % alignment;
                }

                if new_size >= size {
                    block.size = new_size;

                    if block.offset % alignment != 0 {
                        block.offset += alignment - block.offset % alignment;
                    }

                    // check for perfect match
                    if block.size != size {
                        // create new empty block at the end
                        new_block = Some(Block::new(
                            self.memory,
                            block.offset + size,
                            block.size - size,
                        ));

                        // set block size
                        block.size = size;
                    }

                    block.used = true;

                    result_block = Some(block.clone());

                    break;
                }
            }
        }

        if let Some(block) = new_block {
            self.blocks.push(block);
        }

        Ok(result_block)
    }

    pub fn deallocate(&mut self, block: &Block) {
        debug_assert!(self.contains(block));

        let internal_block = self
            .blocks
            .iter_mut()
            .find(|b| *b == block)
            .expect("wrong chunk!");

        internal_block.used = false;
    }

    pub fn contains(&self, block: &Block) -> bool {
        self.blocks.contains(block)
    }

    pub fn memory_type_index(&self) -> u32 {
        self.memory_type_index
    }
}
