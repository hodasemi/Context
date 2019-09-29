pub use VkPipelineColorBlendStateCreateFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkPipelineColorBlendStateCreateFlags {
    VK_PIPELINE_COLOR_BLEND_STATE_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkPipelineColorBlendStateCreateFlagBits(u32);
SetupVkFlags!(
    VkPipelineColorBlendStateCreateFlags,
    VkPipelineColorBlendStateCreateFlagBits
);
