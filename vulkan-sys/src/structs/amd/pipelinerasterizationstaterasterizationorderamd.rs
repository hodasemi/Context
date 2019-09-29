use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
pub struct VkPipelineRasterizationStateRasterizationOrderAMD {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub rasterizationOrder: VkRasterizationOrderAMD,
}

impl VkPipelineRasterizationStateRasterizationOrderAMD {
    pub fn new(rasterization_order: VkRasterizationOrderAMD) -> Self {
        VkPipelineRasterizationStateRasterizationOrderAMD {
            sType: VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD,
            pNext: ptr::null(),
            rasterizationOrder: rasterization_order,
        }
    }
}
