use crate::prelude::*;

use crate::impl_pnext;

use std::os::raw::c_void;
use std::ptr;

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
