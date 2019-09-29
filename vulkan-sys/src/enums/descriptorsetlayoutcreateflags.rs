pub use VkDescriptorSetLayoutCreateFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkDescriptorSetLayoutCreateFlags {
    VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR = 0x0000_0001,
    VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT_EXT = 0x0000_0002,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkDescriptorSetLayoutCreateFlagBits(u32);
SetupVkFlags!(
    VkDescriptorSetLayoutCreateFlags,
    VkDescriptorSetLayoutCreateFlagBits
);
