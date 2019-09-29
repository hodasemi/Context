use crate::prelude::*;

use super::super::{c_char_to_vkstring, raw_to_slice};
use crate::impl_pnext;

use std::ffi::CStr;
use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::os::raw::{c_char, c_double, c_ulong, c_void};
use std::ptr;
use std::slice;

#[repr(C)]
#[derive(Debug)]
pub struct VkPhysicalDeviceFeatures2KHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub features: VkPhysicalDeviceFeatures,
}

impl VkPhysicalDeviceFeatures2KHR {
    pub fn new(features: VkPhysicalDeviceFeatures) -> Self {
        VkPhysicalDeviceFeatures2KHR {
            sType: VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR,
            pNext: ptr::null(),
            features,
        }
    }

    pub fn features(&self) -> VkPhysicalDeviceFeatures {
        self.features
    }
}

impl Default for VkPhysicalDeviceFeatures2KHR {
    fn default() -> Self {
        VkPhysicalDeviceFeatures2KHR::new(VkPhysicalDeviceFeatures::default())
    }
}

impl_pnext!(
    VkPhysicalDeviceFeatures2KHR,
    VkPhysicalDeviceDescriptorIndexingFeaturesEXT
);
