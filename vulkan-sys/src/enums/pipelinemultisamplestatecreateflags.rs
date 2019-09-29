pub use VkPipelineMultisampleStateCreateFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkPipelineMultisampleStateCreateFlags {
    VK_PIPELINE_MULTISAMPLE_STATE_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkPipelineMultisampleStateCreateFlagBits(u32);
SetupVkFlags!(
    VkPipelineMultisampleStateCreateFlags,
    VkPipelineMultisampleStateCreateFlagBits
);
