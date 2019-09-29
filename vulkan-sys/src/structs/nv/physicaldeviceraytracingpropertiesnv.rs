use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkPhysicalDeviceRayTracingPropertiesNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub shaderGroupHandleSize: u32,
    pub maxRecursionDepth: u32,
    pub maxShaderGroupStride: u32,
    pub shaderGroupBaseAlignment: u32,
    pub maxGeometryCount: u64,
    pub maxInstanceCount: u64,
    pub maxTriangleCount: u64,
    pub maxDescriptorSetAccelerationStructures: u32,
}

impl VkPhysicalDeviceRayTracingPropertiesNV {
    pub fn new(
        shader_group_handle_size: u32,
        max_recursion_depth: u32,
        max_shader_group_stride: u32,
        shader_group_base_alignment: u32,
        max_geometry_count: u64,
        max_instance_count: u64,
        max_triangle_count: u64,
        max_descriptor_set_acceleration_structures: u32,
    ) -> Self {
        VkPhysicalDeviceRayTracingPropertiesNV {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV,
            pNext: ptr::null(),
            shaderGroupHandleSize: shader_group_handle_size,
            maxRecursionDepth: max_recursion_depth,
            maxShaderGroupStride: max_shader_group_stride,
            shaderGroupBaseAlignment: shader_group_base_alignment,
            maxGeometryCount: max_geometry_count,
            maxInstanceCount: max_instance_count,
            maxTriangleCount: max_triangle_count,
            maxDescriptorSetAccelerationStructures: max_descriptor_set_acceleration_structures,
        }
    }
}

impl Default for VkPhysicalDeviceRayTracingPropertiesNV {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0)
    }
}
