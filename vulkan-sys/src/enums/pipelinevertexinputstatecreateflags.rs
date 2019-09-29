pub use VkPipelineVertexInputStateCreateFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkPipelineVertexInputStateCreateFlags {
    VK_PIPELINE_VERTEX_INPUT_STATE_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkPipelineVertexInputStateCreateFlagBits(u32);
SetupVkFlags!(
    VkPipelineVertexInputStateCreateFlags,
    VkPipelineVertexInputStateCreateFlagBits
);
