pub use VkDescriptorBindingFlagsEXT::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkDescriptorBindingFlagsEXT {
    VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT_EXT = 0x0000_0001,
    VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT_EXT = 0x0000_0002,
    VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT_EXT = 0x0000_0004,
    VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT_EXT = 0x0000_0008,
    VK_DESCRIPTOR_BINDING_FLAG_BITS_MAX_ENUM_EXT = 0x7FFF_FFFF,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkDescriptorBindingFlagBitsEXT(u32);
SetupVkFlags!(VkDescriptorBindingFlagsEXT, VkDescriptorBindingFlagBitsEXT);
