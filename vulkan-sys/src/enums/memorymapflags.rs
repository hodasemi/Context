pub use VkMemoryMapFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkMemoryMapFlags {
    VK_MEMORY_MAP_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkMemoryMapFlagBits(u32);
SetupVkFlags!(VkMemoryMapFlags, VkMemoryMapFlagBits);
