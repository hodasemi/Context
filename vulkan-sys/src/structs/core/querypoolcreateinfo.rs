use crate::prelude::*;

use super::super::{c_char_to_vkstring, raw_to_slice};

use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::os::raw::{c_char, c_double, c_ulong, c_void};
use std::ptr;
use std::slice;

#[repr(C)]
#[derive(Debug)]
pub struct VkQueryPoolCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkQueryPoolCreateFlagBits,
    pub queryType: VkQueryType,
    pub queryCount: u32,
    pub pipelineStatistics: VkQueryPipelineStatisticFlagBits,
}

impl VkQueryPoolCreateInfo {
    pub fn new<T, U>(
        flags: T,
        query_type: VkQueryType,
        query_count: u32,
        pipeline_statistics: U,
    ) -> Self
    where
        T: Into<VkQueryPoolCreateFlagBits>,
        U: Into<VkQueryPipelineStatisticFlagBits>,
    {
        VkQueryPoolCreateInfo {
            sType: VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            queryType: query_type,
            queryCount: query_count,
            pipelineStatistics: pipeline_statistics.into(),
        }
    }
}
