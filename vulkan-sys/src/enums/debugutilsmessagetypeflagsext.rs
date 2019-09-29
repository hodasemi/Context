pub use VkDebugUtilsMessageTypeFlagsEXT::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkDebugUtilsMessageTypeFlagsEXT {
    VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT = 0x00000001,
    VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT = 0x00000002,
    VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT = 0x00000004,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkDebugUtilsMessageTypeFlagBitsEXT(u32);
SetupVkFlags!(
    VkDebugUtilsMessageTypeFlagsEXT,
    VkDebugUtilsMessageTypeFlagBitsEXT
);
