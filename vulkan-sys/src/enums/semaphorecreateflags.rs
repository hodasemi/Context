pub use VkSemaphoreCreateFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkSemaphoreCreateFlags {
    VK_SEMAPHORE_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkSemaphoreCreateFlagBits(u32);
SetupVkFlags!(VkSemaphoreCreateFlags, VkSemaphoreCreateFlagBits);
