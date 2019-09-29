pub use VkCopyAccelerationStructureModeNV::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkCopyAccelerationStructureModeNV {
    VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_NV = 0,
    VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_NV = 1,
}
