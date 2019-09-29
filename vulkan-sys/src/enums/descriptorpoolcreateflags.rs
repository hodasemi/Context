pub use VkDescriptorPoolCreateFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkDescriptorPoolCreateFlags {
    VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT = 0x0000_0001,
    VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT_EXT = 0x0000_0002,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkDescriptorPoolCreateFlagBits(u32);
SetupVkFlags!(VkDescriptorPoolCreateFlags, VkDescriptorPoolCreateFlagBits);
