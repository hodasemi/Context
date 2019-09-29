pub use VkPipelineTessellationStateCreateFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkPipelineTessellationStateCreateFlags {
    VK_PIPELINE_TESSELATION_STATE_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkPipelineTessellationStateCreateFlagBits(u32);
SetupVkFlags!(
    VkPipelineTessellationStateCreateFlags,
    VkPipelineTessellationStateCreateFlagBits
);
