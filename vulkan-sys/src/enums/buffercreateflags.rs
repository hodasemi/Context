pub use VkBufferCreateFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkBufferCreateFlags {
    VK_BUFFER_CREATE_SPARSE_BINDING_BIT = 0x0000_0001,
    VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT = 0x0000_0002,
    VK_BUFFER_CREATE_SPARSE_ALIASED_BIT = 0x0000_0004,
    VK_BUFFER_CREATE_PROTECTED_BIT = 0x0000_0008,
    VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT = 0x0000_0010,
    VK_BUFFER_CREATE_FLAG_BITS_MAX_ENUM = 0x7FFF_FFFF,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkBufferCreateFlagBits(u32);
SetupVkFlags!(VkBufferCreateFlags, VkBufferCreateFlagBits);
