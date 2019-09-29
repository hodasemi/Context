pub use VkDeviceQueueCreateFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkDeviceQueueCreateFlags {
    VK_DEVICE_QUEUE_CREATE_PROTECTED_BIT = 0x0000_0001,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkDeviceQueueCreateFlagBits(u32);
SetupVkFlags!(VkDeviceQueueCreateFlags, VkDeviceQueueCreateFlagBits);
