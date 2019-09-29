use crate::prelude::*;

use std::ptr;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VkDescriptorSetLayoutBinding {
    pub binding: u32,
    pub descriptorType: VkDescriptorType,
    pub descriptorCount: u32,
    pub stageFlagBits: VkShaderStageFlagBits,
    pub pImmutableSamplers: *const VkSampler,
}

impl VkDescriptorSetLayoutBinding {
    pub fn new<T>(binding: u32, descriptor_type: VkDescriptorType, stage_flags: T) -> Self
    where
        T: Into<VkShaderStageFlagBits>,
    {
        VkDescriptorSetLayoutBinding {
            binding,
            descriptorType: descriptor_type,
            descriptorCount: 1,
            stageFlagBits: stage_flags.into(),
            pImmutableSamplers: ptr::null(),
        }
    }

    pub fn new_array<T>(
        binding: u32,
        count: u32,
        descriptor_type: VkDescriptorType,
        stage_flags: T,
    ) -> Self
    where
        T: Into<VkShaderStageFlagBits>,
    {
        VkDescriptorSetLayoutBinding {
            binding,
            descriptorType: descriptor_type,
            descriptorCount: count,
            stageFlagBits: stage_flags.into(),
            pImmutableSamplers: ptr::null(),
        }
    }

    pub fn set_immutable_samplers<'a, 'b: 'a>(&'a mut self, immutable_samplers: &'b [VkSampler]) {
        self.pImmutableSamplers = if immutable_samplers.is_empty() {
            ptr::null()
        } else {
            immutable_samplers.as_ptr()
        };
    }
}
