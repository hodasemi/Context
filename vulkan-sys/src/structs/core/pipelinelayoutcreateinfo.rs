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
pub struct VkPipelineLayoutCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineLayoutCreateFlagBits,
    pub setLayoutCount: u32,
    pub pSetLayouts: *const VkDescriptorSetLayout,
    pub pushConstantRangeCount: u32,
    pub pPushConstantRanges: *const VkPushConstantRange,
}

impl VkPipelineLayoutCreateInfo {
    pub fn new<T>(
        flags: T,
        set_layouts: &[VkDescriptorSetLayout],
        push_constant_ranges: &[VkPushConstantRange],
    ) -> Self
    where
        T: Into<VkPipelineLayoutCreateFlagBits>,
    {
        VkPipelineLayoutCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            setLayoutCount: set_layouts.len() as u32,
            pSetLayouts: set_layouts.as_ptr(),
            pushConstantRangeCount: push_constant_ranges.len() as u32,
            pPushConstantRanges: push_constant_ranges.as_ptr(),
        }
    }
}
