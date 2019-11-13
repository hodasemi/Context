use crate::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct VkVertexInputAttributeDescription {
    pub location: u32,
    pub binding: u32,
    pub format: VkFormat,
    pub offset: u32,
}
