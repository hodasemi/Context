use crate::prelude::*;
use utilities::prelude::*;

use std::mem::size_of;
use std::os::raw::c_void;
use std::slice;

#[derive(Clone, PartialEq, Debug)]
pub struct Block {
    memory: VkDeviceMemory,

    pub(crate) offset: VkDeviceSize,
    pub(crate) size: VkDeviceSize,

    pub(crate) used: bool,

    mapping: Option<*mut c_void>,
}

unsafe impl Sync for Block {}
unsafe impl Send for Block {}

impl Block {
    pub fn new(memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize) -> Self {
        Block {
            memory,

            offset,
            size,
            used: false,

            mapping: None,
        }
    }

    pub(crate) fn set_host_ptr(&mut self, mapping: Option<*mut c_void>) {
        self.mapping = mapping;
    }

    pub(crate) fn memory(&self) -> VkDeviceMemory {
        self.memory
    }

    pub(crate) fn map<U: Clone>(
        &self,
        length: VkDeviceSize,
    ) -> VerboseResult<VkMappedMemory<'_, U>> {
        let ptr = self
            .mapping
            .ok_or("block is not allocated with host visibility!")?;

        let size = length * size_of::<U>() as VkDeviceSize;

        debug_assert!(size <= self.size);

        let slice = unsafe { slice::from_raw_parts_mut(ptr as *mut U, length as usize) };
        Ok(VkMappedMemory::new(slice))
    }
}
