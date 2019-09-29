use crate::prelude::*;

use std::os::raw::{c_char, c_void};

#[repr(C)]
#[derive(Debug)]
pub struct VkDebugUtilsLabelEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pLabelName: *const c_char,
    pub color: [f32; 4],
}
