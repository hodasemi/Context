pub use VkQueueFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkQueueFlags {
    VK_QUEUE_GRAPHICS_BIT = 0x0000_0001,
    VK_QUEUE_COMPUTE_BIT = 0x0000_0002,
    VK_QUEUE_TRANSFER_BIT = 0x0000_0004,
    VK_QUEUE_SPARSE_BINDING_BIT = 0x0000_0008,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkQueueFlagBits(u32);
SetupVkFlags!(VkQueueFlags, VkQueueFlagBits);
