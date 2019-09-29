use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkRayTracingShaderGroupCreateInfoNV {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub r#type: VkRayTracingShaderGroupTypeNV,
    pub generalShader: u32,
    pub closestHitShader: u32,
    pub anyHitShader: u32,
    pub intersectionShader: u32,
}

impl VkRayTracingShaderGroupCreateInfoNV {
    pub fn new(
        r#type: VkRayTracingShaderGroupTypeNV,
        general_shader: u32,
        closest_hit_shader: u32,
        any_hit_shader: u32,
        intersection_shader: u32,
    ) -> Self {
        VkRayTracingShaderGroupCreateInfoNV {
            sType: VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV,
            pNext: ptr::null(),
            r#type,
            generalShader: general_shader,
            closestHitShader: closest_hit_shader,
            anyHitShader: any_hit_shader,
            intersectionShader: intersection_shader,
        }
    }
}
