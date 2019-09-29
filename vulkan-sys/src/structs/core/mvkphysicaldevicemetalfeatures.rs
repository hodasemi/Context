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
pub struct VkMVKPhysicalDeviceMetalFeatures {
    pub depthClipMode: VkBool32,
    pub indirectDrawing: VkBool32,
    pub baseVertexInstanceDrawing: VkBool32,
    pub maxVertexBufferCount: u32,
    pub maxFragmentBufferCount: u32,
    pub bufferAlignment: VkDeviceSize,
    pub pushConstantsAlignment: VkDeviceSize,
}
