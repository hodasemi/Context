pub use VkRayTracingShaderGroupTypeNV::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkRayTracingShaderGroupTypeNV {
    VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_NV = 0,
    VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_NV = 1,
    VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV = 2,
}
