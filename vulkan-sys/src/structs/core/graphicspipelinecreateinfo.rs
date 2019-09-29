use crate::prelude::*;

use super::super::{c_char_to_vkstring, raw_to_slice};

use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::os::raw::{c_char, c_double, c_ulong, c_void};
use std::ptr;
use std::slice;

#[repr(C)]
#[derive(Debug)]
pub struct VkGraphicsPipelineCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineCreateFlagBits,
    pub stageCount: u32,
    pub pStages: *const VkPipelineShaderStageCreateInfo,
    pub pVertexInputState: *const VkPipelineVertexInputStateCreateInfo,
    pub pInputAssemblyState: *const VkPipelineInputAssemblyStateCreateInfo,
    pub pTessellationState: *const VkPipelineTessellationStateCreateInfo,
    pub pViewportState: *const VkPipelineViewportStateCreateInfo,
    pub pRasterizationState: *const VkPipelineRasterizationStateCreateInfo,
    pub pMultisampleState: *const VkPipelineMultisampleStateCreateInfo,
    pub pDepthStencilState: *const VkPipelineDepthStencilStateCreateInfo,
    pub pColorBlendState: *const VkPipelineColorBlendStateCreateInfo,
    pub pDynamicState: *const VkPipelineDynamicStateCreateInfo,
    pub layout: VkPipelineLayout,
    pub renderPass: VkRenderPass,
    pub subpass: u32,
    pub basePipelineHandle: VkPipeline,
    pub basePipelineIndex: i32,
}

impl VkGraphicsPipelineCreateInfo {
    pub fn new<T>(
        flags: T,
        stages: &[VkPipelineShaderStageCreateInfo],
        vertex_input: Option<&VkPipelineVertexInputStateCreateInfo>,
        input_assembly: Option<&VkPipelineInputAssemblyStateCreateInfo>,
        tesselation: Option<&VkPipelineTessellationStateCreateInfo>,
        viewport: Option<&VkPipelineViewportStateCreateInfo>,
        rasterization: &VkPipelineRasterizationStateCreateInfo,
        multisample: Option<&VkPipelineMultisampleStateCreateInfo>,
        depth_stencil: Option<&VkPipelineDepthStencilStateCreateInfo>,
        color_blend: Option<&VkPipelineColorBlendStateCreateInfo>,
        dynamic: Option<&VkPipelineDynamicStateCreateInfo>,
        layout: VkPipelineLayout,
        renderpass: VkRenderPass,
        subpass: u32,
    ) -> Self
    where
        T: Into<VkPipelineCreateFlagBits>,
    {
        VkGraphicsPipelineCreateInfo {
            sType: VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            stageCount: stages.len() as u32,
            pStages: stages.as_ptr(),
            pVertexInputState: match vertex_input {
                Some(state) => state as *const _,
                None => ptr::null(),
            },
            pInputAssemblyState: match input_assembly {
                Some(state) => state as *const _,
                None => ptr::null(),
            },
            pTessellationState: match tesselation {
                Some(state) => state as *const _,
                None => ptr::null(),
            },
            pViewportState: match viewport {
                Some(state) => state as *const _,
                None => ptr::null(),
            },
            pRasterizationState: rasterization,
            pMultisampleState: match multisample {
                Some(state) => state as *const _,
                None => ptr::null(),
            },
            pDepthStencilState: match depth_stencil {
                Some(state) => state as *const _,
                None => ptr::null(),
            },
            pColorBlendState: match color_blend {
                Some(state) => state as *const _,
                None => ptr::null(),
            },
            pDynamicState: match dynamic {
                Some(state) => state as *const _,
                None => ptr::null(),
            },
            layout,
            renderPass: renderpass,
            subpass,
            basePipelineHandle: VkPipeline::default(),
            basePipelineIndex: -1,
        }
    }

    pub fn set_base_pipeline(&mut self, pipeline: VkPipeline, index: i32) {
        self.basePipelineHandle = pipeline;
        self.basePipelineIndex = index;
    }
}
