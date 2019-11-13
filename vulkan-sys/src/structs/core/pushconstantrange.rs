use crate::prelude::*;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VkPushConstantRange {
    pub stageFlagBits: VkShaderStageFlagBits,
    pub offset: u32,
    pub size: u32,
}

impl VkPushConstantRange {
    pub fn new<T>(flags: T, offset: u32, size: u32) -> Self
    where
        T: Into<VkShaderStageFlagBits>,
    {
        VkPushConstantRange {
            stageFlagBits: flags.into(),
            offset,
            size,
        }
    }
}
