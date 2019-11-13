use crate::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct VkVertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub inputRate: VkVertexInputRate,
}
