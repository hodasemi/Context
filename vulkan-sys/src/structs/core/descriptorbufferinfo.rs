use crate::prelude::*;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VkDescriptorBufferInfo {
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub range: VkDeviceSize,
}
