pub use VkDescriptorPoolResetFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkDescriptorPoolResetFlags {
    VK_DESCRIPTOR_POOL_RESET_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkDescriptorPoolResetFlagBits(u32);
SetupVkFlags!(VkDescriptorPoolResetFlags, VkDescriptorPoolResetFlagBits);
