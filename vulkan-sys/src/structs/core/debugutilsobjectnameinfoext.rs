use crate::prelude::*;

use super::super::c_char_to_vkstring;

use std::os::raw::{c_char, c_void};
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VkDebugUtilsObjectNameInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub objectType: VkObjectType,
    pub objectHandle: u64,
    pub pObjectName: *const c_char,
}

impl VkDebugUtilsObjectNameInfoEXT {
    pub fn new<'a, 'b: 'a>(
        object_type: VkObjectType,
        object_handle: u64,
        object_name: &'b VkString,
    ) -> Self {
        VkDebugUtilsObjectNameInfoEXT {
            sType: VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT,
            pNext: ptr::null(),
            objectType: object_type,
            objectHandle: object_handle,
            pObjectName: object_name.as_ptr(),
        }
    }

    pub fn object_name(&self) -> Result<VkString, String> {
        c_char_to_vkstring(self.pObjectName)
    }
}
