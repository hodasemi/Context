use crate::prelude::*;
use crate::SetupU64Conv;

use std::mem;
use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkPipelineCache(u64);
SetupU64Conv!(VkPipelineCache);
