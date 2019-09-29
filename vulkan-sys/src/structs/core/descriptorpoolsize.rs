use crate::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct VkDescriptorPoolSize {
    pub ty: VkDescriptorType,
    pub descriptorCount: u32,
}
