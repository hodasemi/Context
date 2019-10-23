use crate::prelude::*;

use super::super::{c_char_to_vkstring, raw_to_slice};

use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::os::raw::{c_char, c_double, c_ulong, c_void};
use std::ptr;
use std::slice;

#[repr(C)]
pub struct VkPhysicalDeviceProperties {
    pub apiVersion: u32,
    pub driverVersion: u32,
    pub vendorID: u32,
    pub deviceID: u32,
    pub deviceType: VkPhysicalDeviceType,
    pub deviceName: [c_char; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],
    pub pipelineCacheUUID: [u8; VK_UUID_SIZE as usize],
    pub limits: VkPhysicalDeviceLimits,
    pub sparseProperties: VkPhysicalDeviceSparseProperties,
}

impl Default for VkPhysicalDeviceProperties {
    fn default() -> Self {
        VkPhysicalDeviceProperties {
            apiVersion: 0,
            driverVersion: 0,
            vendorID: 0,
            deviceID: 0,
            deviceType: VK_PHYSICAL_DEVICE_TYPE_OTHER,
            deviceName: [0; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],
            pipelineCacheUUID: [0; VK_UUID_SIZE as usize],
            limits: VkPhysicalDeviceLimits::default(),
            sparseProperties: VkPhysicalDeviceSparseProperties::default(),
        }
    }
}

impl fmt::Debug for VkPhysicalDeviceProperties {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let device_name_cstr = unsafe { CStr::from_ptr(self.deviceName.as_ptr()) };
        let device_name = device_name_cstr.to_str().unwrap();

        write!(
            f,
            "VkPhysicalDeviceProperties {{ apiVersion: {:?}, driverVersion: {:?}, vendorID: {:?}, deviceID: {:?}, deviceType: {:?}, deviceName: {:?}, pipelineCacheUUID: {:?}, limits: {:?}, sparseProperties: {:?} }}",
            self.apiVersion, self.driverVersion, self.vendorID, self.deviceID, self.deviceType, device_name, self.pipelineCacheUUID, self.limits, self.sparseProperties
        )
    }
}
