use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkPipelineInputAssemblyStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineInputAssemblyStateCreateFlagBits,
    pub topology: VkPrimitiveTopology,
    pub primitiveRestartEnable: VkBool32,
}

impl VkPipelineInputAssemblyStateCreateInfo {
    pub fn new<T>(
        flags: T,
        topology: VkPrimitiveTopology,
        primitive_restart_enable: bool,
    ) -> VkPipelineInputAssemblyStateCreateInfo
    where
        T: Into<VkPipelineInputAssemblyStateCreateFlagBits>,
    {
        VkPipelineInputAssemblyStateCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            topology,
            primitiveRestartEnable: primitive_restart_enable.into(),
        }
    }
}
