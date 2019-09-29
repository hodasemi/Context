pub use VkDebugUtilsMessengerCallbackDataFlagsEXT::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkDebugUtilsMessengerCallbackDataFlagsEXT {
    VK_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_NULL_BIT = 0,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkDebugUtilsMessengerCallbackDataFlagBitsEXT(u32);
SetupVkFlags!(
    VkDebugUtilsMessengerCallbackDataFlagsEXT,
    VkDebugUtilsMessengerCallbackDataFlagBitsEXT
);
