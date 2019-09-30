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
pub struct VkQueueFamilyProperties {
    pub queueFlagBits: VkQueueFlagBits,
    pub queueCount: u32,
    pub timestampValidBits: u32,
    pub minImageTransferGranularity: VkExtent3D,
}