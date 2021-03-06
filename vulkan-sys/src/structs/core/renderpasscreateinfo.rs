use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkRenderPassCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkRenderPassCreateFlagBits,
    pub attachmentCount: u32,
    pub pAttachments: *const VkAttachmentDescription,
    pub subpassCount: u32,
    pub pSubpasses: *const VkSubpassDescription,
    pub dependencyCount: u32,
    pub pDependencies: *const VkSubpassDependency,
}

impl VkRenderPassCreateInfo {
    pub fn new<T>(
        flags: T,
        attachments: &[VkAttachmentDescription],
        subpasses: &[VkSubpassDescription],
        dependencies: &[VkSubpassDependency],
    ) -> Self
    where
        T: Into<VkRenderPassCreateFlagBits>,
    {
        VkRenderPassCreateInfo {
            sType: VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            attachmentCount: attachments.len() as u32,
            pAttachments: attachments.as_ptr(),
            subpassCount: subpasses.len() as u32,
            pSubpasses: subpasses.as_ptr(),
            dependencyCount: dependencies.len() as u32,
            pDependencies: dependencies.as_ptr(),
        }
    }
}
