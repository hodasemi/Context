use crate::prelude::*;

use super::super::{c_char_to_vkstring, raw_to_slice};

use std::os::raw::{c_char, c_void};
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkDebugUtilsMessengerCallbackDataEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDebugUtilsMessengerCallbackDataFlagBitsEXT,
    pub pMessageIdName: *const c_char,
    pub messageIdNumber: i32,
    pub pMessage: *const c_char,
    pub queueLabelCount: u32,
    pub pQueueLabels: *const VkDebugUtilsLabelEXT,
    pub cmdBufLabelCount: u32,
    pub pCmdBufLabels: *const VkDebugUtilsLabelEXT,
    pub objectCount: u32,
    pub pObjects: *const VkDebugUtilsObjectNameInfoEXT,
}

impl VkDebugUtilsMessengerCallbackDataEXT {
    pub fn new<'a, 'b: 'a, 'c: 'a, 'd: 'a, 'e: 'a, T>(
        flags: T,
        message_id_name: &VkString,
        message_id_number: i32,
        message: &'b VkString,
        queue_labels: &'c [VkDebugUtilsLabelEXT],
        cmd_buf_labels: &'d [VkDebugUtilsLabelEXT],
        objects: &'e [VkDebugUtilsObjectNameInfoEXT],
    ) -> Self
    where
        T: Into<VkDebugUtilsMessengerCallbackDataFlagBitsEXT>,
    {
        VkDebugUtilsMessengerCallbackDataEXT {
            sType: VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT,
            pNext: ptr::null(),
            flags: flags.into(),
            pMessageIdName: message_id_name.as_ptr(),
            messageIdNumber: message_id_number,
            pMessage: message.as_ptr(),
            queueLabelCount: queue_labels.len() as u32,
            pQueueLabels: queue_labels.as_ptr(),
            cmdBufLabelCount: cmd_buf_labels.len() as u32,
            pCmdBufLabels: cmd_buf_labels.as_ptr(),
            objectCount: objects.len() as u32,
            pObjects: objects.as_ptr(),
        }
    }

    pub fn objects(&self) -> &[VkDebugUtilsObjectNameInfoEXT] {
        raw_to_slice(self.pObjects, self.objectCount)
    }

    pub fn message(&self) -> Result<VkString, String> {
        c_char_to_vkstring(self.pMessage)
    }
}
