use crate::prelude::*;

use super::super::{c_char_to_vkstring, raw_to_slice};

use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::os::raw::{c_char, c_double, c_ulong, c_void};
use std::ptr;
use std::slice;

#[repr(C)]
#[derive(Debug, Clone, Default, Copy)]
pub struct VkMemoryType {
    pub propertyFlagBits: VkMemoryPropertyFlagBits,
    pub heapIndex: u32,
}
