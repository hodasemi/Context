use crate::prelude::*;

use std::os::raw::c_void;

#[repr(C)]
#[derive(Debug)]
pub struct VkDisplayModeCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDisplayModeCreateFlagBitsKHR,
    pub parameters: VkDisplayModeParametersKHR,
}
