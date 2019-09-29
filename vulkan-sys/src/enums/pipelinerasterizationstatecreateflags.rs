pub use VkPipelineRasterizationStateCreateFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkPipelineRasterizationStateCreateFlags {
    VK_PIPELINE_RASTERIZATION_STATE_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkPipelineRasterizationStateCreateFlagBits(u32);
SetupVkFlags!(
    VkPipelineRasterizationStateCreateFlags,
    VkPipelineRasterizationStateCreateFlagBits
);
