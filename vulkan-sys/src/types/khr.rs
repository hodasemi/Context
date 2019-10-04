use crate::SetupU64Conv;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkDescriptorUpdateTemplateKHR(u64);
SetupU64Conv!(VkDescriptorUpdateTemplateKHR);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkDisplayKHR(u64);
SetupU64Conv!(VkDisplayKHR);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkDisplayModeKHR(u64);
SetupU64Conv!(VkDisplayModeKHR);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkSurfaceKHR(u64);
SetupU64Conv!(VkSurfaceKHR);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkSwapchainKHR(u64);
SetupU64Conv!(VkSwapchainKHR);
