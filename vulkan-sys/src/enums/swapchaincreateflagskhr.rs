pub use VkSwapchainCreateFlagsKHR::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkSwapchainCreateFlagsKHR {
    VK_SWAPCHAIN_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR = 0x0000_0001,
    VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR = 0x0000_0002,
    VK_SWAPCHAIN_CREATE_MUTABLE_FORMAT_BIT_KHR = 0x0000_0004,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkSwapchainCreateFlagBitsKHR(u32);
SetupVkFlags!(VkSwapchainCreateFlagsKHR, VkSwapchainCreateFlagBitsKHR);
