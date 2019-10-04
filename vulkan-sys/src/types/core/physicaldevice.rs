use crate::prelude::*;
use crate::SetupUSizeConv;

use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkPhysicalDevice(usize);
SetupUSizeConv!(VkPhysicalDevice);
