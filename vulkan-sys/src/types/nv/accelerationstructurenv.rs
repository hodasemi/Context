use crate::prelude::*;
use crate::SetupU64Conv;

use core::ffi::c_void;
use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkAccelerationStructureNV(u64);
SetupU64Conv!(VkAccelerationStructureNV);
