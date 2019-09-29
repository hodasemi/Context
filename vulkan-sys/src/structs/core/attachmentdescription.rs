use crate::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct VkAttachmentDescription {
    pub flags: VkAttachmentDescriptionFlagBits,
    pub format: VkFormat,
    pub samples: VkSampleCountFlagBits,
    pub loadOp: VkAttachmentLoadOp,
    pub storeOp: VkAttachmentStoreOp,
    pub stencilLoadOp: VkAttachmentLoadOp,
    pub stencilStoreOp: VkAttachmentStoreOp,
    pub initialLayout: VkImageLayout,
    pub finalLayout: VkImageLayout,
}

impl VkAttachmentDescription {
    pub fn new<T, U>(
        flags: T,
        format: VkFormat,
        samples: U,
        load_op: VkAttachmentLoadOp,
        store_op: VkAttachmentStoreOp,
        stencil_load_op: VkAttachmentLoadOp,
        stencil_store_op: VkAttachmentStoreOp,
        initial_layout: VkImageLayout,
        final_layout: VkImageLayout,
    ) -> Self
    where
        T: Into<VkAttachmentDescriptionFlagBits>,
        U: Into<VkSampleCountFlagBits>,
    {
        VkAttachmentDescription {
            flags: flags.into(),
            format,
            samples: samples.into(),
            loadOp: load_op,
            storeOp: store_op,
            stencilLoadOp: stencil_load_op,
            stencilStoreOp: stencil_store_op,
            initialLayout: initial_layout,
            finalLayout: final_layout,
        }
    }
}
