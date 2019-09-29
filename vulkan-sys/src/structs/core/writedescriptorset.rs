use crate::impl_pnext;
use crate::prelude::*;

use super::super::{c_char_to_vkstring, raw_to_slice};

use std::ffi::CStr;
use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::os::raw::{c_char, c_double, c_ulong, c_void};
use std::ptr;
use std::slice;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VkWriteDescriptorSet {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub dstSet: VkDescriptorSet,
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
    pub descriptorType: VkDescriptorType,
    pub pImageInfo: *const VkDescriptorImageInfo,
    pub pBufferInfo: *const VkDescriptorBufferInfo,
    pub pTexelBufferView: *const VkBufferView,
}

impl VkWriteDescriptorSet {
    pub fn new(
        set: VkDescriptorSet,
        binding: u32,
        array_element: u32,
        descriptor_type: VkDescriptorType,
    ) -> Self {
        VkWriteDescriptorSet {
            sType: VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
            pNext: ptr::null(),
            dstSet: set,
            dstBinding: binding,
            dstArrayElement: array_element,
            descriptorCount: 0,
            descriptorType: descriptor_type,
            pImageInfo: ptr::null(),
            pBufferInfo: ptr::null(),
            pTexelBufferView: ptr::null(),
        }
    }

    pub fn set_image_infos<'a, 'b: 'a>(&'a mut self, image_infos: &'b [VkDescriptorImageInfo]) {
        self.descriptorCount = image_infos.len() as u32;
        self.pImageInfo = image_infos.as_ptr() as *const _;
    }

    pub fn set_buffer_infos<'a, 'b: 'a>(&'a mut self, buffer_infos: &'b [VkDescriptorBufferInfo]) {
        self.descriptorCount = buffer_infos.len() as u32;
        self.pBufferInfo = buffer_infos.as_ptr() as *const _;
    }

    pub fn set_texel_buffer_views<'a, 'b: 'a>(
        &'a mut self,
        texel_buffer_views: &'b [VkBufferView],
    ) {
        self.descriptorCount = texel_buffer_views.len() as u32;
        self.pTexelBufferView = texel_buffer_views.as_ptr() as *const _;
    }
}

impl_pnext!(
    VkWriteDescriptorSet,
    VkWriteDescriptorSetAccelerationStructureNV
);
