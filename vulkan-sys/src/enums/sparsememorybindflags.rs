pub use VkSparseMemoryBindFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkSparseMemoryBindFlags {
    VK_SPARSE_MEMORY_BIND_METADATA_BIT = 0x0000_0001,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkSparseMemoryBindFlagBits(u32);
SetupVkFlags!(VkSparseMemoryBindFlags, VkSparseMemoryBindFlagBits);
