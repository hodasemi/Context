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
