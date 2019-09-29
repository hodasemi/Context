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
pub struct VkMVKDeviceConfiguration {
    pub supportDisplayContentsScale: VkBool32,
    pub imageFlipY: VkBool32,
    pub shaderConversionFlipFragmentY: VkBool32,
    pub shaderConversionFlipVertexY: VkBool32,
    pub shaderConversionLogging: VkBool32,
    pub performanceTracking: VkBool32,
    pub performanceLoggingFrameCount: u32,
}
