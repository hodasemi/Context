use crate::prelude::*;

use std::os::raw::c_void;

pub type PFN_vkAllocationFunction =
    extern "system" fn(*mut c_void, usize, usize, VkSystemAllocationScope) -> *mut c_void;

pub type PFN_vkReallocationFunction = extern "system" fn(
    *mut c_void,
    *mut c_void,
    usize,
    usize,
    VkSystemAllocationScope,
) -> *mut c_void;

pub type PFN_vkFreeFunction = extern "system" fn(*mut c_void, *mut c_void);
pub type PFN_vkInternalAllocationNotification = extern "system" fn(
    *mut c_void,
    usize,
    VkInternalAllocationType,
    VkSystemAllocationScope,
) -> *mut c_void;

pub type PFN_vkInternalFreeNotification = extern "system" fn(
    *mut c_void,
    usize,
    VkInternalAllocationType,
    VkSystemAllocationScope,
) -> *mut c_void;

#[repr(C)]
pub struct VkAllocationCallbacks {
    pub pUserData: *mut c_void,
    pub pfnAllocation: PFN_vkAllocationFunction,
    pub pfnReallocation: PFN_vkReallocationFunction,
    pub pfnFree: PFN_vkFreeFunction,
    pub pfnInternalAllocation: PFN_vkInternalAllocationNotification,
    pub pfnInternalFree: PFN_vkInternalFreeNotification,
}
