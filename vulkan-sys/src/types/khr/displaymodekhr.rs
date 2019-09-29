use crate::SetupU64Conv;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkDisplayModeKHR(u64);
SetupU64Conv!(VkDisplayModeKHR);
