pub use VkInstanceCreateFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkInstanceCreateFlags {
    VK_INSTANCE_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkInstanceCreateFlagBits(u32);
SetupVkFlags!(VkInstanceCreateFlags, VkInstanceCreateFlagBits);
