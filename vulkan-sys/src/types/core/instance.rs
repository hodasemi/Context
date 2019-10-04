use crate::prelude::*;
use crate::SetupUSizeConv;

use std::mem;
use std::ptr;

use std::os::raw::c_void;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkInstance(usize);
SetupUSizeConv!(VkInstance);
