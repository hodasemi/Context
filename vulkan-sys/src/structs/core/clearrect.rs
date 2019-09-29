use crate::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct VkClearRect {
    pub VkRect: VkRect2D,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}
