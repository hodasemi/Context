use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkPipelineVertexInputStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineVertexInputStateCreateFlagBits,
    pub vertexBindingDescriptionCount: u32,
    pub pVertexBindingDescriptions: *const VkVertexInputBindingDescription,
    pub vertexAttributeDescriptionCount: u32,
    pub pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription,
}

impl VkPipelineVertexInputStateCreateInfo {
    pub fn new<T>(
        flags: T,
        vertex_binding_descriptions: &[VkVertexInputBindingDescription],
        vertex_attrbiute_descriptions: &[VkVertexInputAttributeDescription],
    ) -> VkPipelineVertexInputStateCreateInfo
    where
        T: Into<VkPipelineVertexInputStateCreateFlagBits>,
    {
        VkPipelineVertexInputStateCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            vertexBindingDescriptionCount: vertex_binding_descriptions.len() as u32,
            pVertexBindingDescriptions: vertex_binding_descriptions.as_ptr(),
            vertexAttributeDescriptionCount: vertex_attrbiute_descriptions.len() as u32,
            pVertexAttributeDescriptions: vertex_attrbiute_descriptions.as_ptr(),
        }
    }
}
