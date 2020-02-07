use crate::prelude::*;
use utilities::prelude::*;

use super::block::Block;

use std::os::raw::c_void;
use std::sync::Arc;

pub struct Chunk {
    device: Arc<Device>,

    memory: VkDeviceMemory,
    memory_type_index: u32,
    size: VkDeviceSize,

    blocks: Vec<Block>,

    mapping: Option<*mut c_void>,
}

unsafe impl Sync for Chunk {}
unsafe impl Send for Chunk {}

impl Chunk {
    pub fn new(
        device: Arc<Device>,
        memory_type_index: u32,
        size: VkDeviceSize,
    ) -> VerboseResult<Self> {
        let memory_ci = VkMemoryAllocateInfo::new(size, memory_type_index);

        let memory = device.allocate_memory(&memory_ci)?;

        let mapping = if (device.physical_device().memory_properties().memoryTypes
            [memory_type_index as usize]
            .propertyFlagBits
            & VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT)
            == VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT
        {
            Some(device.map_memory_raw(memory, 0, VK_WHOLE_SIZE, VK_MEMORY_MAP_NULL_BIT)?)
        } else {
            None
        };

        Ok(Chunk {
            device,

            memory,
            memory_type_index,
            size,

            blocks: vec![Block::new(memory, 0, size)],

            mapping,
        })
    }

    pub fn allocate(
        &mut self,
        size: VkDeviceSize,
        alignment: VkDeviceSize,
    ) -> VerboseResult<Option<Block>> {
        if self.size < size {
            return Ok(None);
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

                    // We compute offset and size that care about alignment (for this Block)
                    if block.offset % alignment != 0 {
                        block.offset += alignment - block.offset % alignment;
                    }

                    // ptr address
                    if let Some(mapping) = self.mapping {
                        block.set_host_ptr(Some(unsafe { mapping.offset(block.offset as isize) }));
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

impl Drop for Chunk {
    fn drop(&mut self) {
        if self.mapping.is_some() {
            self.mapping = None;
            self.device.unmap_memory(self.memory);
        }
    }
}
