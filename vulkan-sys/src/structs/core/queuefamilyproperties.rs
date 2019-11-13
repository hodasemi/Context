use crate::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct VkQueueFamilyProperties {
    pub queueFlagBits: VkQueueFlagBits,
    pub queueCount: u32,
    pub timestampValidBits: u32,
    pub minImageTransferGranularity: VkExtent3D,
}
