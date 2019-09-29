pub use VkPipelineDynamicStateCreateFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkPipelineDynamicStateCreateFlags {
    VK_PIPELINE_DYNAMIC_STATE_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkPipelineDynamicStateCreateFlagBits(u32);
SetupVkFlags!(
    VkPipelineDynamicStateCreateFlags,
    VkPipelineDynamicStateCreateFlagBits
);
