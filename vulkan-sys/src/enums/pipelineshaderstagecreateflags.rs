pub use VkPipelineShaderStageCreateFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkPipelineShaderStageCreateFlags {
    VK_PIPELINE_SHADER_STAGE_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkPipelineShaderStageCreateFlagBits(u32);
SetupVkFlags!(
    VkPipelineShaderStageCreateFlags,
    VkPipelineShaderStageCreateFlagBits
);
