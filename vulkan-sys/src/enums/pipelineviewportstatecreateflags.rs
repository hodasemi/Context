pub use VkPipelineViewportStateCreateFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkPipelineViewportStateCreateFlags {
    VK_PIPELINE_VIEWPORT_STATE_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkPipelineViewportStateCreateFlagBits(u32);
SetupVkFlags!(
    VkPipelineViewportStateCreateFlags,
    VkPipelineViewportStateCreateFlagBits
);
