pub use VkMemoryHeapFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkMemoryHeapFlags {
    VK_MEMORY_HEAP_DEVICE_LOCAL_BIT = 0x0000_0001,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash, Default)]
pub struct VkMemoryHeapFlagBits(u32);
SetupVkFlags!(VkMemoryHeapFlags, VkMemoryHeapFlagBits);
