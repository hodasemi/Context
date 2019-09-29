use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
pub struct VkDebugUtilsMessengerCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDebugUtilsMessengerCreateFlagBitsEXT,
    pub messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    pub messageType: VkDebugUtilsMessageTypeFlagBitsEXT,
    pub pfnUserCallback: PFN_vkDebugUtilsMessengerCallbackEXT,
    pub pUserData: *mut c_void,
}

impl VkDebugUtilsMessengerCreateInfoEXT {
    pub fn new<T, U, V>(
        flags: T,
        message_severity: U,
        message_type: V,
        user_callback: PFN_vkDebugUtilsMessengerCallbackEXT,
    ) -> Self
    where
        T: Into<VkDebugUtilsMessengerCreateFlagBitsEXT>,
        U: Into<VkDebugUtilsMessageSeverityFlagBitsEXT>,
        V: Into<VkDebugUtilsMessageTypeFlagBitsEXT>,
    {
        VkDebugUtilsMessengerCreateInfoEXT {
            sType: VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
            pNext: ptr::null(),
            flags: flags.into(),
            messageSeverity: message_severity.into(),
            messageType: message_type.into(),
            pfnUserCallback: user_callback,
            pUserData: ptr::null_mut(),
        }
    }

    pub fn set_user_data<'a, 'b: 'a, W>(&'a mut self, user_data: &'b mut W) {
        self.pUserData = user_data as *mut W as *mut c_void;
    }
}
