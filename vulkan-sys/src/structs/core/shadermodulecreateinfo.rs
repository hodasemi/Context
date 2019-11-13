use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkShaderModuleCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkShaderModuleCreateFlagBits,
    pub codeSize: usize,
    pub pCode: *const u32,
}

impl VkShaderModuleCreateInfo {
    pub fn new<T>(flags: T, code: &[u8]) -> Self
    where
        T: Into<VkShaderModuleCreateFlagBits>,
    {
        VkShaderModuleCreateInfo {
            sType: VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            codeSize: code.len(),
            pCode: code.as_ptr() as *const u32,
        }
    }
}
