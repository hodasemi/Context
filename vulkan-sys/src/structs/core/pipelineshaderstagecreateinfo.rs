use crate::prelude::*;

use std::os::raw::{c_char, c_void};
use std::ptr;

const SHADER_ENTRY: &str = "main\0";

#[repr(C)]
#[derive(Debug)]
pub struct VkPipelineShaderStageCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineShaderStageCreateFlagBits,
    pub stage: VkShaderStageFlags,
    pub module: VkShaderModule,
    pub pName: *const c_char,
    pub pSpecializationInfo: *const VkSpecializationInfo,
}

impl VkPipelineShaderStageCreateInfo {
    pub fn new<T>(
        flags: T,
        stage: VkShaderStageFlags,
        shader_module: VkShaderModule,
        entry_function_name: &VkString,
    ) -> VkPipelineShaderStageCreateInfo
    where
        T: Into<VkPipelineShaderStageCreateFlagBits>,
    {
        VkPipelineShaderStageCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            stage,
            module: shader_module,
            pName: entry_function_name.as_ptr(),
            pSpecializationInfo: ptr::null(),
        }
    }

    pub fn main<T>(
        flags: T,
        stage: VkShaderStageFlags,
        shader_module: VkShaderModule,
    ) -> VkPipelineShaderStageCreateInfo
    where
        T: Into<VkPipelineShaderStageCreateFlagBits>,
    {
        VkPipelineShaderStageCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            stage,
            module: shader_module,
            pName: SHADER_ENTRY.as_ptr() as *const c_char,
            pSpecializationInfo: ptr::null(),
        }
    }

    pub fn vertex(shader_module: VkShaderModule) -> VkPipelineShaderStageCreateInfo {
        Self::main(
            VK_PIPELINE_SHADER_STAGE_CREATE_NULL_BIT,
            VK_SHADER_STAGE_VERTEX_BIT,
            shader_module,
        )
    }

    pub fn fragment(shader_module: VkShaderModule) -> VkPipelineShaderStageCreateInfo {
        Self::main(
            VK_PIPELINE_SHADER_STAGE_CREATE_NULL_BIT,
            VK_SHADER_STAGE_FRAGMENT_BIT,
            shader_module,
        )
    }

    pub fn geometry(shader_module: VkShaderModule) -> VkPipelineShaderStageCreateInfo {
        Self::main(
            VK_PIPELINE_SHADER_STAGE_CREATE_NULL_BIT,
            VK_SHADER_STAGE_GEOMETRY_BIT,
            shader_module,
        )
    }

    pub fn tesselation_control(shader_module: VkShaderModule) -> VkPipelineShaderStageCreateInfo {
        Self::main(
            VK_PIPELINE_SHADER_STAGE_CREATE_NULL_BIT,
            VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT,
            shader_module,
        )
    }

    pub fn tesselation_evaluation(
        shader_module: VkShaderModule,
    ) -> VkPipelineShaderStageCreateInfo {
        Self::main(
            VK_PIPELINE_SHADER_STAGE_CREATE_NULL_BIT,
            VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT,
            shader_module,
        )
    }

    pub fn compute(shader_module: VkShaderModule) -> VkPipelineShaderStageCreateInfo {
        Self::main(
            VK_PIPELINE_SHADER_STAGE_CREATE_NULL_BIT,
            VK_SHADER_STAGE_COMPUTE_BIT,
            shader_module,
        )
    }

    pub fn ray_generation(shader_module: VkShaderModule) -> VkPipelineShaderStageCreateInfo {
        Self::main(
            VK_PIPELINE_SHADER_STAGE_CREATE_NULL_BIT,
            VK_SHADER_STAGE_RAYGEN_BIT_NV,
            shader_module,
        )
    }

    pub fn closest_hit(shader_module: VkShaderModule) -> VkPipelineShaderStageCreateInfo {
        Self::main(
            VK_PIPELINE_SHADER_STAGE_CREATE_NULL_BIT,
            VK_SHADER_STAGE_CLOSEST_HIT_BIT_NV,
            shader_module,
        )
    }

    pub fn miss(shader_module: VkShaderModule) -> VkPipelineShaderStageCreateInfo {
        Self::main(
            VK_PIPELINE_SHADER_STAGE_CREATE_NULL_BIT,
            VK_SHADER_STAGE_MISS_BIT_NV,
            shader_module,
        )
    }

    pub fn intersection(shader_module: VkShaderModule) -> VkPipelineShaderStageCreateInfo {
        Self::main(
            VK_PIPELINE_SHADER_STAGE_CREATE_NULL_BIT,
            VK_SHADER_STAGE_INTERSECTION_BIT_NV,
            shader_module,
        )
    }

    pub fn any_hit(shader_module: VkShaderModule) -> VkPipelineShaderStageCreateInfo {
        Self::main(
            VK_PIPELINE_SHADER_STAGE_CREATE_NULL_BIT,
            VK_SHADER_STAGE_ANY_HIT_BIT_NV,
            shader_module,
        )
    }

    pub fn set_specialization_info(&mut self, specialization_info: &VkSpecializationInfo) {
        self.pSpecializationInfo = specialization_info as *const VkSpecializationInfo;
    }
}
