pub use VkShaderModuleCreateFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkShaderModuleCreateFlags {
    VK_SHADER_MODULE_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkShaderModuleCreateFlagBits(u32);
SetupVkFlags!(VkShaderModuleCreateFlags, VkShaderModuleCreateFlagBits);
