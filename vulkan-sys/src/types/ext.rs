use crate::SetupU64Conv;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkDebugReportCallbackEXT(u64);
SetupU64Conv!(VkDebugReportCallbackEXT);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkDebugUtilsMessengerEXT(u64);
SetupU64Conv!(VkDebugUtilsMessengerEXT);
