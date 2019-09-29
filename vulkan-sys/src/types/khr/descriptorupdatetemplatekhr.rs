use crate::SetupU64Conv;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkDescriptorUpdateTemplateKHR(u64);
SetupU64Conv!(VkDescriptorUpdateTemplateKHR);
