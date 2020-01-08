use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkSpecializationInfo {
    pub mapEntryCount: u32,
    pub pMapEntries: *const VkSpecializationMapEntry,
    pub dataSize: usize,
    pub pData: *const c_void,
}

impl VkSpecializationInfo {
    pub fn empty() -> Self {
        VkSpecializationInfo {
            mapEntryCount: 0,
            pMapEntries: ptr::null(),
            dataSize: 0,
            pData: ptr::null(),
        }
    }

    pub fn new<T>(data: &T, map_entries: &[VkSpecializationMapEntry]) -> Self {
        VkSpecializationInfo {
            mapEntryCount: map_entries.len() as u32,
            pMapEntries: map_entries.as_ptr(),
            dataSize: std::mem::size_of::<T>(),
            pData: data as *const T as *const c_void,
        }
    }

    pub fn set_map_entries(&mut self, map_entries: &[VkSpecializationMapEntry]) {
        self.mapEntryCount = map_entries.len() as u32;
        self.pMapEntries = map_entries.as_ptr();
    }

    pub fn set_data<T>(&mut self, data: &T) {
        self.dataSize = std::mem::size_of::<T>();
        self.pData = data as *const T as *const c_void;
    }
}
