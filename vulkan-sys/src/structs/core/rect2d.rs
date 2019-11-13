use crate::prelude::*;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VkRect2D {
    pub offset: VkOffset2D,
    pub extent: VkExtent2D,
}
