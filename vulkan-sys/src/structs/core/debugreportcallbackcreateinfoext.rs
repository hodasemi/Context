use crate::prelude::*;
use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
pub struct VkDebugReportCallbackCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDebugReportFlagBitsEXT,
    pub pfnCallback: PFN_vkDebugReportCallbackEXT,
    pub pUserData: *mut c_void,
}

impl VkDebugReportCallbackCreateInfoEXT {
    pub fn new<T>(flags: T, callback: PFN_vkDebugReportCallbackEXT) -> Self
    where
        T: Into<VkDebugReportFlagBitsEXT>,
    {
        VkDebugReportCallbackCreateInfoEXT {
            sType: VK_STRUCTURE_TYPE_DEBUG_REPORT_CREATE_INFO_EXT,
            pNext: ptr::null(),
            flags: flags.into(),
            pfnCallback: callback,
            pUserData: ptr::null_mut(),
        }
    }

    pub fn set_user_data<'a, 'b: 'a, T>(&'a mut self, user_data: &'b mut T) {
        self.pUserData = user_data as *mut T as *mut c_void;
    }
}
