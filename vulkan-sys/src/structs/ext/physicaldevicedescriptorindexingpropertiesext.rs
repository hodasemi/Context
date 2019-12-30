use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkPhysicalDeviceDescriptorIndexingPropertiesEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub maxUpdateAfterBindDescriptorsInAllPools: u32,
    pub shaderUniformBufferArrayNonUniformIndexingNative: VkBool32,
    pub shaderSampledImageArrayNonUniformIndexingNative: VkBool32,
    pub shaderStorageBufferArrayNonUniformIndexingNative: VkBool32,
    pub shaderStorageImageArrayNonUniformIndexingNative: VkBool32,
    pub shaderInputAttachmentArrayNonUniformIndexingNative: VkBool32,
    pub robustBufferAccessUpdateAfterBind: VkBool32,
    pub quadDivergentImplicitLod: VkBool32,
    pub maxPerStageDescriptorUpdateAfterBindSamplers: u32,
    pub maxPerStageDescriptorUpdateAfterBindUniformBuffers: u32,
    pub maxPerStageDescriptorUpdateAfterBindStorageBuffers: u32,
    pub maxPerStageDescriptorUpdateAfterBindSampledImages: u32,
    pub maxPerStageDescriptorUpdateAfterBindStorageImages: u32,
    pub maxPerStageDescriptorUpdateAfterBindInputAttachments: u32,
    pub maxPerStageUpdateAfterBindResources: u32,
    pub maxDescriptorSetUpdateAfterBindSamplers: u32,
    pub maxDescriptorSetUpdateAfterBindUniformBuffers: u32,
    pub maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: u32,
    pub maxDescriptorSetUpdateAfterBindStorageBuffers: u32,
    pub maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: u32,
    pub maxDescriptorSetUpdateAfterBindSampledImages: u32,
    pub maxDescriptorSetUpdateAfterBindStorageImages: u32,
    pub maxDescriptorSetUpdateAfterBindInputAttachments: u32,
}

impl Default for VkPhysicalDeviceDescriptorIndexingPropertiesEXT {
    fn default() -> Self {
        VkPhysicalDeviceDescriptorIndexingPropertiesEXT {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES_EXT,
            pNext: ptr::null(),
            maxUpdateAfterBindDescriptorsInAllPools: 0,
            shaderUniformBufferArrayNonUniformIndexingNative: VK_FALSE,
            shaderSampledImageArrayNonUniformIndexingNative: VK_FALSE,
            shaderStorageBufferArrayNonUniformIndexingNative: VK_FALSE,
            shaderStorageImageArrayNonUniformIndexingNative: VK_FALSE,
            shaderInputAttachmentArrayNonUniformIndexingNative: VK_FALSE,
            robustBufferAccessUpdateAfterBind: VK_FALSE,
            quadDivergentImplicitLod: VK_FALSE,
            maxPerStageDescriptorUpdateAfterBindSamplers: 0,
            maxPerStageDescriptorUpdateAfterBindUniformBuffers: 0,
            maxPerStageDescriptorUpdateAfterBindStorageBuffers: 0,
            maxPerStageDescriptorUpdateAfterBindSampledImages: 0,
            maxPerStageDescriptorUpdateAfterBindStorageImages: 0,
            maxPerStageDescriptorUpdateAfterBindInputAttachments: 0,
            maxPerStageUpdateAfterBindResources: 0,
            maxDescriptorSetUpdateAfterBindSamplers: 0,
            maxDescriptorSetUpdateAfterBindUniformBuffers: 0,
            maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: 0,
            maxDescriptorSetUpdateAfterBindStorageBuffers: 0,
            maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: 0,
            maxDescriptorSetUpdateAfterBindSampledImages: 0,
            maxDescriptorSetUpdateAfterBindStorageImages: 0,
            maxDescriptorSetUpdateAfterBindInputAttachments: 0,
        }
    }
}

unsafe impl Sync for VkPhysicalDeviceDescriptorIndexingPropertiesEXT {}
unsafe impl Send for VkPhysicalDeviceDescriptorIndexingPropertiesEXT {}
