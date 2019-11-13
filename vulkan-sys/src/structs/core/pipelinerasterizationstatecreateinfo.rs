use crate::impl_pnext;
use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkPipelineRasterizationStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineRasterizationStateCreateFlagBits,
    pub depthClampEnable: VkBool32,
    pub rasterizerDiscardEnable: VkBool32,
    pub polygonMode: VkPolygonMode,
    pub cullMode: VkCullModeFlags,
    pub frontFace: VkFrontFace,
    pub depthBiasEnable: VkBool32,
    pub depthBiasConstantFactor: f32,
    pub depthBiasClamp: f32,
    pub depthBiasSlopeFactor: f32,
    pub lineWidth: f32,
}

impl VkPipelineRasterizationStateCreateInfo {
    pub fn new<T>(
        flags: T,
        depth_clamp_enable: bool,
        rasterization_discard_enable: bool,
        polygon_mode: VkPolygonMode,
        cull_mode: VkCullModeFlags,
        front_face: VkFrontFace,
        depth_bias_enable: bool,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32,
        line_width: f32,
    ) -> VkPipelineRasterizationStateCreateInfo
    where
        T: Into<VkPipelineRasterizationStateCreateFlagBits>,
    {
        VkPipelineRasterizationStateCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            depthClampEnable: depth_clamp_enable.into(),
            rasterizerDiscardEnable: rasterization_discard_enable.into(),
            polygonMode: polygon_mode,
            cullMode: cull_mode,
            frontFace: front_face,
            depthBiasEnable: depth_bias_enable.into(),
            depthBiasConstantFactor: depth_bias_constant_factor,
            depthBiasClamp: depth_bias_clamp,
            depthBiasSlopeFactor: depth_bias_slope_factor,
            lineWidth: line_width,
        }
    }
}

impl_pnext!(
    VkPipelineRasterizationStateCreateInfo,
    VkPipelineRasterizationStateRasterizationOrderAMD
);
