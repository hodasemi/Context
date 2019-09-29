use crate::prelude::*;

use super::super::{c_char_to_vkstring, raw_to_slice};

use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::os::raw::{c_char, c_double, c_ulong, c_void};
use std::ptr;
use std::slice;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VkPushConstantRange {
    pub stageFlagBits: VkShaderStageFlagBits,
    pub offset: u32,
    pub size: u32,
}

impl VkPushConstantRange {
    pub fn new<T>(flags: T, offset: u32, size: u32) -> Self
    where
        T: Into<VkShaderStageFlagBits>,
    {
        VkPushConstantRange {
            stageFlagBits: flags.into(),
            offset,
            size,
        }
    }
}
