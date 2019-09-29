pub use VkDeviceCreateFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkDeviceCreateFlags {
    VK_DEVICE_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkDeviceCreateFlagBits(u32);
SetupVkFlags!(VkDeviceCreateFlags, VkDeviceCreateFlagBits);
