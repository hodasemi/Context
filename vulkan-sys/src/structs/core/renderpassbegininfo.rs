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
