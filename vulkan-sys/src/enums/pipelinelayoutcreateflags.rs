pub use VkPipelineLayoutCreateFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkPipelineLayoutCreateFlags {
    VK_PIPELINE_LAYOUT_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkPipelineLayoutCreateFlagBits(u32);
SetupVkFlags!(VkPipelineLayoutCreateFlags, VkPipelineLayoutCreateFlagBits);
