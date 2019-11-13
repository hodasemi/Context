use crate::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct VkFormatProperties {
    pub linearTilingFeatures: VkFormatFeatureFlagBits,
    pub optimalTilingFeatures: VkFormatFeatureFlagBits,
    pub bufferFeatures: VkFormatFeatureFlagBits,
}
