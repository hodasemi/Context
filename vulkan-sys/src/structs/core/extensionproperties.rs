use crate::prelude::*;

use super::super::c_char_to_vkstring;

use std::os::raw::c_char;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExtensionProperties {
    pub extensionName: [c_char; VK_MAX_EXTENSION_NAME_SIZE as usize],
    pub specVersion: u32,
}

impl VkExtensionProperties {
    pub fn extension_name(&self) -> Result<VkString, String> {
        c_char_to_vkstring(&self.extensionName as *const c_char)
    }
}

impl Default for VkExtensionProperties {
    fn default() -> Self {
        VkExtensionProperties {
            extensionName: [0; VK_MAX_EXTENSION_NAME_SIZE as usize],
            specVersion: 0,
        }
    }
}
