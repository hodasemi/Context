use crate::impl_pnext;
use crate::prelude::*;

use std::marker::PhantomData;
use std::os::raw::{c_char, c_void};
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkDeviceCreateInfo<'a> {
    lt: PhantomData<&'a ()>,
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkDeviceCreateFlagBits,
    pub queueCreateInfoCount: u32,
    pub pQueueCreateInfos: *const VkDeviceQueueCreateInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const c_char,
    pub pEnabledFeatures: *const VkPhysicalDeviceFeatures,
}

impl<'a> VkDeviceCreateInfo<'a> {
    pub fn new<T>(
        flags: T,
        queue_create_info: &'a [VkDeviceQueueCreateInfo],
        enabled_extension_names: &'a VkNames,
        enabled_features: &'a VkPhysicalDeviceFeatures,
    ) -> Self
    where
        T: Into<VkDeviceCreateFlagBits>,
    {
        VkDeviceCreateInfo {
            lt: PhantomData,
            sType: VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            queueCreateInfoCount: queue_create_info.len() as u32,
            pQueueCreateInfos: queue_create_info.as_ptr(),
            enabledLayerCount: 0,
            ppEnabledLayerNames: ptr::null(),
            enabledExtensionCount: enabled_extension_names.c_names().len() as u32,
            ppEnabledExtensionNames: enabled_extension_names.c_names().as_ptr(),
            pEnabledFeatures: enabled_features as *const _,
        }
    }
}

impl_pnext!(
    VkDeviceCreateInfo<'_>,
    VkPhysicalDeviceDescriptorIndexingFeaturesEXT
);
