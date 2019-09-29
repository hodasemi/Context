use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkExternalMemoryBufferCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub handleTypes: VkExternalMemoryHandleTypeFlags,
}

impl VkExternalMemoryBufferCreateInfo {
    pub fn new<T>(handle_types: T) -> Self
    where
        T: Into<VkExternalMemoryHandleTypeFlags>,
    {
        VkExternalMemoryBufferCreateInfo {
            sType: VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO,
            pNext: ptr::null(),
            handleTypes: handle_types.into(),
        }
    }
}
