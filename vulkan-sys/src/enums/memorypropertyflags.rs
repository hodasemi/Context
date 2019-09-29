pub use VkMemoryPropertyFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkMemoryPropertyFlags {
    VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT = 0x0000_0001,
    VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT = 0x0000_0002,
    VK_MEMORY_PROPERTY_HOST_COHERENT_BIT = 0x0000_0004,
    VK_MEMORY_PROPERTY_HOST_CACHED_BIT = 0x0000_0008,
    VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT = 0x0000_0010,
    VK_MEMORY_PROPERTY_PROTECTED_BIT = 0x0000_0020,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash, Default)]
pub struct VkMemoryPropertyFlagBits(u32);
SetupVkFlags!(VkMemoryPropertyFlags, VkMemoryPropertyFlagBits);
