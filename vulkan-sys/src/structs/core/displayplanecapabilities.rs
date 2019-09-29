use crate::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct VkDisplayPlaneCapabilitiesKHR {
    pub supportedAlpha: VkDisplayPlaneAlphaFlagBitsKHR,
    pub minSrcPosition: VkOffset2D,
    pub maxSrcPosition: VkOffset2D,
    pub minSrcExtent: VkExtent2D,
    pub maxSrcExtent: VkExtent2D,
    pub minDstPosition: VkOffset2D,
    pub maxDstPosition: VkOffset2D,
    pub minDstExtent: VkExtent2D,
    pub maxDstExtent: VkExtent2D,
}
