use crate::prelude::*;
use crate::SetupUSizeConv;

use core::ffi::c_void;
use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkDevice(usize);
SetupUSizeConv!(VkDevice);
