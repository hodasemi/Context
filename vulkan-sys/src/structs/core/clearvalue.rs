use crate::prelude::*;

use std::mem;

#[repr(C)]
#[derive(Debug)]
pub struct VkClearValue(VkClearColorValue);

impl VkClearValue {
    #[inline]
    pub fn as_color(&self) -> &VkClearColorValue {
        &self.0
    }
    #[inline]
    pub fn as_depth_stencil(&self) -> &VkClearDepthStencilValue {
        unsafe { mem::transmute(&self.0) }
    }

    #[inline]
    pub fn color(val: VkClearColorValue) -> VkClearValue {
        VkClearValue(val)
    }
    #[inline]
    pub fn depth_stencil(val: VkClearDepthStencilValue) -> VkClearValue {
        let val = (val, [0u32, 0u32]);
        VkClearValue(unsafe { mem::transmute(val) })
    }
}
