use crate::prelude::*;

use std::mem;
use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkPipelineCacheCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCacheCreateFlagBits,
    pub initialDataSize: usize,
    pub pInitialData: *const c_void,
}

impl VkPipelineCacheCreateInfo {
    pub fn new<T>(flags: T) -> Self
    where
        T: Into<VkPipelineCacheCreateFlagBits>,
    {
        VkPipelineCacheCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            initialDataSize: 0,
            pInitialData: ptr::null(),
        }
    }

    pub fn set_data<T>(&mut self, data: &T) {
        self.initialDataSize = mem::size_of::<T>();
        self.pInitialData = data as *const T as *const c_void;
    }
}
