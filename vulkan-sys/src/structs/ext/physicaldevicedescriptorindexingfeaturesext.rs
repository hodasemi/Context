use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkPhysicalDeviceDescriptorIndexingFeaturesEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub shaderInputAttachmentArrayDynamicIndexing: VkBool32,
    pub shaderUniformTexelBufferArrayDynamicIndexing: VkBool32,
    pub shaderStorageTexelBufferArrayDynamicIndexing: VkBool32,
    pub shaderUniformBufferArrayNonUniformIndexing: VkBool32,
    pub shaderSampledImageArrayNonUniformIndexing: VkBool32,
    pub shaderStorageBufferArrayNonUniformIndexing: VkBool32,
    pub shaderStorageImageArrayNonUniformIndexing: VkBool32,
    pub shaderInputAttachmentArrayNonUniformIndexing: VkBool32,
    pub shaderUniformTexelBufferArrayNonUniformIndexing: VkBool32,
    pub shaderStorageTexelBufferArrayNonUniformIndexing: VkBool32,
    pub descriptorBindingUniformBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingSampledImageUpdateAfterBind: VkBool32,
    pub descriptorBindingStorageImageUpdateAfterBind: VkBool32,
    pub descriptorBindingStorageBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingUniformTexelBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingStorageTexelBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingUpdateUnusedWhilePending: VkBool32,
    pub descriptorBindingPartiallyBound: VkBool32,
    pub descriptorBindingVariableDescriptorCount: VkBool32,
    pub runtimeDescriptorArray: VkBool32,
}

impl Default for VkPhysicalDeviceDescriptorIndexingFeaturesEXT {
    fn default() -> Self {
        VkPhysicalDeviceDescriptorIndexingFeaturesEXT {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT,
            pNext: ptr::null(),
            shaderInputAttachmentArrayDynamicIndexing: VK_FALSE,
            shaderUniformTexelBufferArrayDynamicIndexing: VK_FALSE,
            shaderStorageTexelBufferArrayDynamicIndexing: VK_FALSE,
            shaderUniformBufferArrayNonUniformIndexing: VK_FALSE,
            shaderSampledImageArrayNonUniformIndexing: VK_FALSE,
            shaderStorageBufferArrayNonUniformIndexing: VK_FALSE,
            shaderStorageImageArrayNonUniformIndexing: VK_FALSE,
            shaderInputAttachmentArrayNonUniformIndexing: VK_FALSE,
            shaderUniformTexelBufferArrayNonUniformIndexing: VK_FALSE,
            shaderStorageTexelBufferArrayNonUniformIndexing: VK_FALSE,
            descriptorBindingUniformBufferUpdateAfterBind: VK_FALSE,
            descriptorBindingSampledImageUpdateAfterBind: VK_FALSE,
            descriptorBindingStorageImageUpdateAfterBind: VK_FALSE,
            descriptorBindingStorageBufferUpdateAfterBind: VK_FALSE,
            descriptorBindingUniformTexelBufferUpdateAfterBind: VK_FALSE,
            descriptorBindingStorageTexelBufferUpdateAfterBind: VK_FALSE,
            descriptorBindingUpdateUnusedWhilePending: VK_FALSE,
            descriptorBindingPartiallyBound: VK_FALSE,
            descriptorBindingVariableDescriptorCount: VK_FALSE,
            runtimeDescriptorArray: VK_FALSE,
        }
    }
}
