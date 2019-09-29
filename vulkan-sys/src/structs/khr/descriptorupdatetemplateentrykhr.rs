use crate::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct VkDescriptorUpdateTemplateEntryKHR {
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
    pub descriptorType: VkDescriptorType,
    pub offset: usize,
    pub stride: usize,
}
