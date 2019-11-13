use crate::prelude::*;

#[repr(C)]
#[derive(Debug, Default)]
pub struct VkSurfaceCapabilitiesKHR {
    pub minImageCount: u32,
    pub maxImageCount: u32,
    pub currentExtent: VkExtent2D,
    pub minImageExtent: VkExtent2D,
    pub maxImageExtent: VkExtent2D,
    pub maxImageArrayLayers: u32,
    pub supportedTransforms: VkSurfaceTransformFlagBitsKHR,
    pub currentTransform: VkSurfaceTransformFlagBitsKHR,
    pub supportedCompositeAlpha: VkCompositeAlphaFlagBitsKHR,
    pub supportedUsageFlagBits: VkImageUsageFlagBits,
}
