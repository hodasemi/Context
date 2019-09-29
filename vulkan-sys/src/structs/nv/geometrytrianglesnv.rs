use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkGeometryTrianglesNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub vertexData: VkBuffer,
    pub vertexOffset: VkDeviceSize,
    pub vertexCount: u32,
    pub vertexStride: VkDeviceSize,
    pub vertexFormat: VkFormat,
    pub indexData: VkBuffer,
    pub indexOffset: VkDeviceSize,
    pub indexCount: u32,
    pub indexType: VkIndexType,
    pub transformData: VkBuffer,
    pub transformOffset: VkDeviceSize,
}

impl VkGeometryTrianglesNV {
    pub fn new(
        vertex_data: VkBuffer,
        vertex_offset: VkDeviceSize,
        vertex_count: u32,
        vertex_stride: VkDeviceSize,
        vertex_format: VkFormat,
        index_data: VkBuffer,
        index_offset: VkDeviceSize,
        index_count: u32,
        index_type: VkIndexType,
        transform_data: VkBuffer,
        transform_offset: VkDeviceSize,
    ) -> Self {
        VkGeometryTrianglesNV {
            sType: VK_STRUCTURE_TYPE_GEOMETRY_TRIANGLES_NV,
            pNext: ptr::null(),
            vertexData: vertex_data,
            vertexOffset: vertex_offset,
            vertexCount: vertex_count,
            vertexStride: vertex_stride,
            vertexFormat: vertex_format,
            indexData: index_data,
            indexOffset: index_offset,
            indexCount: index_count,
            indexType: index_type,
            transformData: transform_data,
            transformOffset: transform_offset,
        }
    }
}

impl Default for VkGeometryTrianglesNV {
    fn default() -> Self {
        Self::new(
            VkBuffer::default(),
            0,
            0,
            0,
            VkFormat::default(),
            VkBuffer::default(),
            0,
            0,
            VK_INDEX_TYPE_NONE_NV,
            VkBuffer::default(),
            0,
        )
    }
}
