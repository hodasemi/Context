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
pub struct VkPipelineInputAssemblyStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineInputAssemblyStateCreateFlagBits,
    pub topology: VkPrimitiveTopology,
    pub primitiveRestartEnable: VkBool32,
}

impl VkPipelineInputAssemblyStateCreateInfo {
    pub fn new<T>(
        flags: T,
        topology: VkPrimitiveTopology,
        primitive_restart_enable: bool,
    ) -> VkPipelineInputAssemblyStateCreateInfo
    where
        T: Into<VkPipelineInputAssemblyStateCreateFlagBits>,
    {
        VkPipelineInputAssemblyStateCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            topology,
            primitiveRestartEnable: primitive_restart_enable.into(),
        }
    }
}
