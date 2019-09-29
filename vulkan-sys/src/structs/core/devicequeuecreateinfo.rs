use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VkDeviceQueueCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDeviceQueueCreateFlagBits,
    pub queueFamilyIndex: u32,
    pub queueCount: u32,
    pub pQueuePriorities: *const f32,
}

impl VkDeviceQueueCreateInfo {
    pub fn new<T>(flags: T, queue_family_index: u32, queue_priorities: &[f32]) -> Self
    where
        T: Into<VkDeviceQueueCreateFlagBits>,
    {
        VkDeviceQueueCreateInfo {
            sType: VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            queueFamilyIndex: queue_family_index,
            queueCount: queue_priorities.len() as u32,
            pQueuePriorities: queue_priorities.as_ptr(),
        }
    }
}
