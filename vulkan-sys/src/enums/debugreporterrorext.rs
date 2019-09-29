pub use VkDebugReportErrorEXT::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkDebugReportErrorEXT {
    VK_DEBUG_REPORT_ERROR_NONE_EXT = 0,
    VK_DEBUG_REPORT_ERROR_CALLBACK_REF_EXT = 1,
}
