use crate::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct VkSurfaceFormatKHR {
    pub format: VkFormat,
    pub colorSpace: VkColorSpaceKHR,
}
