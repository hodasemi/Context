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
pub struct VkFramebufferCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkFramebufferCreateFlagBits,
    pub renderPass: VkRenderPass,
    pub attachmentCount: u32,
    pub pAttachments: *const VkImageView,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}

impl VkFramebufferCreateInfo {
    pub fn new<T>(
        flags: T,
        renderpass: VkRenderPass,
        attachments: &[VkImageView],
        width: u32,
        height: u32,
        layers: u32,
    ) -> Self
    where
        T: Into<VkFramebufferCreateFlagBits>,
    {
        VkFramebufferCreateInfo {
            sType: VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            renderPass: renderpass,
            attachmentCount: attachments.len() as u32,
            pAttachments: attachments.as_ptr(),
            width,
            height,
            layers,
        }
    }
}
