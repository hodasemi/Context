pub use VkPipelineInputAssemblyStateCreateFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkPipelineInputAssemblyStateCreateFlags {
    VK_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkPipelineInputAssemblyStateCreateFlagBits(u32);
SetupVkFlags!(
    VkPipelineInputAssemblyStateCreateFlags,
    VkPipelineInputAssemblyStateCreateFlagBits
);
