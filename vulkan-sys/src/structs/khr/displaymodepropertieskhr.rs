use crate::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct VkDisplayModePropertiesKHR {
    pub displayMode: VkDisplayModeKHR,
    pub parameters: VkDisplayModeParametersKHR,
}
