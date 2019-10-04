use crate::SetupU64Conv;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkAccelerationStructureNV(u64);
SetupU64Conv!(VkAccelerationStructureNV);
