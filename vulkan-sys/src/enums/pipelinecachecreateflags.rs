pub use VkPipelineCacheCreateFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkPipelineCacheCreateFlags {
    VK_PIPELINE_CACHE_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkPipelineCacheCreateFlagBits(u32);
SetupVkFlags!(VkPipelineCacheCreateFlags, VkPipelineCacheCreateFlagBits);
