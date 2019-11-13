use crate::prelude::*;

use std::os::raw::c_void;

#[repr(C)]
#[derive(Debug)]
pub struct VkSpecializationInfo {
    pub mapEntryCount: u32,
    pub pMapEntries: *const VkSpecializationMapEntry,
    pub dataSize: usize,
    pub pData: *const c_void,
}

impl VkSpecializationInfo {
    pub fn new<T>(data: &T, map_entries: &[VkSpecializationMapEntry]) -> Self {
        VkSpecializationInfo {
            mapEntryCount: map_entries.len() as u32,
            pMapEntries: map_entries.as_ptr(),
            dataSize: std::mem::size_of::<T>(),
            pData: data as *const T as *const c_void,
        }
    }
}
