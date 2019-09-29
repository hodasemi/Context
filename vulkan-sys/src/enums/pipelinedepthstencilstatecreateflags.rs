pub use VkPipelineDepthStencilStateCreateFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkPipelineDepthStencilStateCreateFlags {
    VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkPipelineDepthStencilStateCreateFlagBits(u32);
SetupVkFlags!(
    VkPipelineDepthStencilStateCreateFlags,
    VkPipelineDepthStencilStateCreateFlagBits
);
