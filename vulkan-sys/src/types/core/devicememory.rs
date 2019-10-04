use crate::prelude::*;
use crate::SetupU64Conv;

use std::mem;
use std::ptr;
use std::slice;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkDeviceMemory(u64);
SetupU64Conv!(VkDeviceMemory);
