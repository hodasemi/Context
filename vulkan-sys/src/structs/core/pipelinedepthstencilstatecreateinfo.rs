use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkPipelineDepthStencilStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineDepthStencilStateCreateFlagBits,
    pub depthTestEnable: VkBool32,
    pub depthWriteEnable: VkBool32,
    pub depthCompareOp: VkCompareOp,
    pub depthBoundsTestEnable: VkBool32,
    pub stencilTestEnable: VkBool32,
    pub front: VkStencilOpState,
    pub back: VkStencilOpState,
    pub minDepthBounds: f32,
    pub maxDepthBounds: f32,
}

impl VkPipelineDepthStencilStateCreateInfo {
    pub fn new<T>(
        flags: T,
        depth_test_enable: bool,
        depth_write_enable: bool,
        depth_compare_op: VkCompareOp,
        depth_bounds_test_enable: bool,
        stencil_test_enable: bool,
        front: VkStencilOpState,
        back: VkStencilOpState,
        min_depth_bounds: f32,
        max_depth_bounds: f32,
    ) -> VkPipelineDepthStencilStateCreateInfo
    where
        T: Into<VkPipelineDepthStencilStateCreateFlagBits>,
    {
        VkPipelineDepthStencilStateCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            depthTestEnable: depth_test_enable.into(),
            depthWriteEnable: depth_write_enable.into(),
            depthCompareOp: depth_compare_op,
            depthBoundsTestEnable: depth_bounds_test_enable.into(),
            stencilTestEnable: stencil_test_enable.into(),
            front,
            back,
            minDepthBounds: min_depth_bounds,
            maxDepthBounds: max_depth_bounds,
        }
    }
}
