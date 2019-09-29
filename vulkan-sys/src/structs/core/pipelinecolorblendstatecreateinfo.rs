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
pub struct VkPipelineColorBlendStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkPipelineColorBlendStateCreateFlagBits,
    pub logicOpEnable: VkBool32,
    pub logicOp: VkLogicOp,
    pub attachmentCount: u32,
    pub pAttachments: *const VkPipelineColorBlendAttachmentState,
    pub blendConstants: [f32; 4],
}

impl VkPipelineColorBlendStateCreateInfo {
    pub fn new<T>(
        flags: T,
        logic_op_enable: bool,
        logic_op: VkLogicOp,
        attachments: &[VkPipelineColorBlendAttachmentState],
        blend_constants: [f32; 4],
    ) -> VkPipelineColorBlendStateCreateInfo
    where
        T: Into<VkPipelineColorBlendStateCreateFlagBits>,
    {
        VkPipelineColorBlendStateCreateInfo {
            sType: VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            logicOpEnable: logic_op_enable.into(),
            logicOp: logic_op,
            attachmentCount: attachments.len() as u32,
            pAttachments: attachments.as_ptr(),
            blendConstants: blend_constants,
        }
    }

    pub fn set_attachments(&mut self, attachments: &[VkPipelineColorBlendAttachmentState]) {
        self.attachmentCount = attachments.len() as u32;
        self.pAttachments = attachments.as_ptr();
    }
}
