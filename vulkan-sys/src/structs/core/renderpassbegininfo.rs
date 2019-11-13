use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkRenderPassBeginInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub renderPass: VkRenderPass,
    pub framebuffer: VkFramebuffer,
    pub renderArea: VkRect2D,
    pub clearValueCount: u32,
    pub pClearValues: *const VkClearValue,
}

impl VkRenderPassBeginInfo {
    pub fn new(
        renderpass: VkRenderPass,
        framebuffer: VkFramebuffer,
        render_area: VkRect2D,
        clear_values: &[VkClearValue],
    ) -> Self {
        VkRenderPassBeginInfo {
            sType: VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
            pNext: ptr::null(),
            renderPass: renderpass,
            framebuffer,
            renderArea: render_area,
            clearValueCount: clear_values.len() as u32,
            pClearValues: clear_values.as_ptr(),
        }
    }
}
