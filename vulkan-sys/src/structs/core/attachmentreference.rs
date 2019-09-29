use crate::prelude::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VkAttachmentReference {
    pub attachment: u32,
    pub layout: VkImageLayout,
}
