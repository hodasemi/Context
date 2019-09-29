use crate::prelude::*;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VkDescriptorImageInfo {
    pub sampler: VkSampler,
    pub imageView: VkImageView,
    pub imageLayout: VkImageLayout,
}
