pub use VkSamplerCreateFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkSamplerCreateFlags {
    VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT = 0x0000_0001,
    VK_SAMPLER_CREATE_SUBSAMPLED_COARSE_RECONSTRUCTION_BIT_EXT = 0x0000_0002,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkSamplerCreateFlagBits(u32);
SetupVkFlags!(VkSamplerCreateFlags, VkSamplerCreateFlagBits);
