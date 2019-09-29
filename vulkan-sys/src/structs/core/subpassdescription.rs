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
pub struct VkSubpassDescription {
    pub flags: VkSubpassDescriptionFlagBits,
    pub pipelineBindPoint: VkPipelineBindPoint,
    pub inputAttachmentCount: u32,
    pub pInputAttachments: *const VkAttachmentReference,
    pub colorAttachmentCount: u32,
    pub pColorAttachments: *const VkAttachmentReference,
    pub pResolveAttachments: *const VkAttachmentReference,
    pub pDepthStencilAttachment: *const VkAttachmentReference,
    pub preserveAttachmentCount: u32,
    pub pPreserveAttachments: *const u32,
}

impl VkSubpassDescription {
    pub fn new<T>(
        flags: T,
        input_attachments: &[VkAttachmentReference],
        color_attachments: &[VkAttachmentReference],
        resolve_attachments: &[VkAttachmentReference],
        depth_stencil_attachment: Option<&VkAttachmentReference>,
        preserve_attachments: &[u32],
    ) -> Self
    where
        T: Into<VkSubpassDescriptionFlagBits>,
    {
        VkSubpassDescription {
            flags: flags.into(),
            // the only bit currently supported
            pipelineBindPoint: VK_PIPELINE_BIND_POINT_GRAPHICS,
            inputAttachmentCount: input_attachments.len() as u32,
            pInputAttachments: input_attachments.as_ptr(),
            colorAttachmentCount: color_attachments.len() as u32,
            pColorAttachments: color_attachments.as_ptr(),
            pResolveAttachments: if resolve_attachments.is_empty() {
                ptr::null()
            } else {
                debug_assert!(resolve_attachments.len() == color_attachments.len());
                resolve_attachments.as_ptr()
            },
            pDepthStencilAttachment: match depth_stencil_attachment {
                Some(attachment) => attachment as *const _,
                None => ptr::null(),
            },
            preserveAttachmentCount: preserve_attachments.len() as u32,
            pPreserveAttachments: preserve_attachments.as_ptr(),
        }
    }
}
