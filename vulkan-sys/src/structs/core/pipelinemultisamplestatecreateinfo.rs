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
pub struct VkPipelineMultisampleStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineMultisampleStateCreateFlagBits,
    pub rasterizationSamples: VkSampleCountFlags,
    pub sampleShadingEnable: VkBool32,
    pub minSampleShading: f32,
    pub pSampleMask: *const VkSampleMask,
    pub alphaToCoverageEnable: VkBool32,
    pub alphaToOneEnable: VkBool32,
}

impl VkPipelineMultisampleStateCreateInfo {
    pub fn new<T>(
        flags: T,
        rasterization_samples: VkSampleCountFlags,
        sample_shading_enable: bool,
        min_sample_shading: f32,
        sample_masks: &[VkSampleMask],
        alpha_to_coverage_enable: bool,
        alpha_to_one_enable: bool,
    ) -> VkPipelineMultisampleStateCreateInfo
    where
        T: Into<VkPipelineMultisampleStateCreateFlagBits>,
    {
        VkPipelineMultisampleStateCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            rasterizationSamples: rasterization_samples,
            sampleShadingEnable: sample_shading_enable.into(),
            minSampleShading: min_sample_shading,
            pSampleMask: if sample_masks.is_empty() {
                ptr::null()
            } else {
                let cmp = (sample_masks.len() as f32 / 32.0).ceil() as u32;
                let raster_samples = rasterization_samples as u32;

                debug_assert!(cmp == raster_samples);

                sample_masks.as_ptr()
            },

            alphaToCoverageEnable: alpha_to_coverage_enable.into(),
            alphaToOneEnable: alpha_to_one_enable.into(),
        }
    }
}
