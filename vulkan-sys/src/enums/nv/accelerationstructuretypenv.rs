pub use VkAccelerationStructureTypeNV::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkAccelerationStructureTypeNV {
    VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_NV = 0,
    VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_NV = 1,
}
