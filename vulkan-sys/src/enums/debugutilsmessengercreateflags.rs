pub use VkDebugUtilsMessengerCreateFlagsEXT::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkDebugUtilsMessengerCreateFlagsEXT {
    VK_DEBUG_UTILS_MESSENGER_CREATE_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkDebugUtilsMessengerCreateFlagBitsEXT(u32);
SetupVkFlags!(
    VkDebugUtilsMessengerCreateFlagsEXT,
    VkDebugUtilsMessengerCreateFlagBitsEXT
);
