pub use VkPipelineCreateFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkPipelineCreateFlags {
    VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT = 0x0000_0001,
    VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT = 0x0000_0002,
    VK_PIPELINE_CREATE_DERIVATIVE_BIT = 0x0000_0004,
    VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT = 0x0000_0008,
    VK_PIPELINE_CREATE_DISPATCH_BASE = 0x0000_0010,
    VK_PIPELINE_CREATE_DEFER_COMPILE_BIT_NV = 0x0000_0020,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkPipelineCreateFlagBits(u32);
SetupVkFlags!(VkPipelineCreateFlags, VkPipelineCreateFlagBits);
