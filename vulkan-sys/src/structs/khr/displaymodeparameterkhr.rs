use crate::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct VkDisplayModeParametersKHR {
    pub visibleRegion: VkExtent2D,
    pub refreshRate: u32,
}
