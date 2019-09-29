use crate::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct VkDisplayPlanePropertiesKHR {
    pub currentDisplay: VkDisplayKHR,
    pub currentStackIndex: u32,
}
