use crate::impl_pnext;
use crate::prelude::*;

use std::marker::PhantomData;
use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct VkBufferCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkBufferCreateFlagBits,
    pub size: VkDeviceSize,
    pub usage: VkBufferUsageFlagBits,
    pub sharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
}

impl VkBufferCreateInfo {
    pub fn new<'a, 'b: 'a, T, U>(
        flags: T,
        size: VkDeviceSize,
        usage: U,
        sharing_mode: VkSharingMode,
        queue_family_indices: &'b [u32],
    ) -> VkBufferCreateInfo
    where
        T: Into<VkBufferCreateFlagBits>,
        U: Into<VkBufferUsageFlagBits>,
    {
        VkBufferCreateInfo {
            sType: VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            size,
            usage: usage.into(),
            sharingMode: sharing_mode,
            queueFamilyIndexCount: queue_family_indices.len() as u32,
            pQueueFamilyIndices: queue_family_indices.as_ptr(),
        }
    }
}

impl_pnext!(VkBufferCreateInfo, VkBufferDeviceAddressCreateInfoEXT);
impl_pnext!(VkBufferCreateInfo, VkExternalMemoryBufferCreateInfo);
