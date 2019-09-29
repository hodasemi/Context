use crate::prelude::*;

use super::super::{c_char_to_vkstring, raw_to_slice};

use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::os::raw::{c_char, c_double, c_ulong, c_void};
use std::ptr;
use std::slice;

#[repr(C)]
pub struct VkLayerProperties {
    pub layerName: [c_char; VK_MAX_EXTENSION_NAME_SIZE as usize],
    pub specVersion: u32,
    pub implementationVersion: u32,
    pub description: [c_char; VK_MAX_DESCRIPTION_SIZE as usize],
}

impl VkLayerProperties {
    pub fn layer_name(&self) -> Result<VkString, String> {
        c_char_to_vkstring(&self.layerName as *const c_char)
    }

    pub fn description(&self) -> Result<VkString, String> {
        c_char_to_vkstring(&self.description as *const c_char)
    }
}
