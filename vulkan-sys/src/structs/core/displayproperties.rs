use crate::prelude::*;

use super::super::c_char_to_vkstring;

use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug)]
pub struct VkDisplayPropertiesKHR {
    pub display: VkDisplayKHR,
    pub displayName: *const c_char,
    pub physicalDimensions: VkExtent2D,
    pub physicalResolution: VkExtent2D,
    pub supportedTransforms: VkSurfaceTransformFlagBitsKHR,
    pub planeReorderPossible: VkBool32,
    pub persistentContent: VkBool32,
}

impl VkDisplayPropertiesKHR {
    pub fn display_name(&self) -> Result<VkString, String> {
        c_char_to_vkstring(self.displayName)
    }
}
