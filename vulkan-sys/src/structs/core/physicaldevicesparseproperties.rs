use crate::prelude::*;

#[repr(C)]
#[derive(Debug, Default)]
pub struct VkPhysicalDeviceSparseProperties {
    pub residencyStandard2DBlockShape: VkBool32,
    pub residencyStandard2DMultisampleBlockShape: VkBool32,
    pub residencyStandard3DBlockShape: VkBool32,
    pub residencyAlignedMipSize: VkBool32,
    pub residencyNonResidentStrict: VkBool32,
}
