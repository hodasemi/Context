use crate::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct VkMVKDeviceConfiguration {
    pub supportDisplayContentsScale: VkBool32,
    pub imageFlipY: VkBool32,
    pub shaderConversionFlipFragmentY: VkBool32,
    pub shaderConversionFlipVertexY: VkBool32,
    pub shaderConversionLogging: VkBool32,
    pub performanceTracking: VkBool32,
    pub performanceLoggingFrameCount: u32,
}
