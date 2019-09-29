use crate::prelude::*;

use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkBufferDeviceAddressCreateInfoEXT {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub deviceAddress: VkDeviceSize,
}

impl VkBufferDeviceAddressCreateInfoEXT {
    pub fn new(device_address: VkDeviceSize) -> Self {
        VkBufferDeviceAddressCreateInfoEXT {
            sType: VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT,
            pNext: ptr::null(),
            deviceAddress: device_address,
        }
    }
}
