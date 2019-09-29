use crate::prelude::*;

use super::super::{c_char_to_vkstring, raw_to_slice};

use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::os::raw::{c_char, c_double, c_ulong, c_void};
use std::ptr;
use std::slice;

#[repr(C)]
#[derive(Debug)]
pub struct VkPresentInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const VkSemaphore,
    pub swapchainCount: u32,
    pub pSwapchains: *const VkSwapchainKHR,
    pub pImageIndices: *const u32,
    pub pResults: *mut VkResult,
}

impl VkPresentInfoKHR {
    pub fn new(
        wait_semaphores: &[VkSemaphore],
        swapchains: &[VkSwapchainKHR],
        image_indices: &[u32],
        results: &mut [VkResult],
    ) -> Self {
        debug_assert!(image_indices.len() == swapchains.len());

        VkPresentInfoKHR {
            sType: VK_STRUCTURE_TYPE_PRESENT_INFO_KHR,
            pNext: ptr::null(),
            waitSemaphoreCount: wait_semaphores.len() as u32,
            pWaitSemaphores: wait_semaphores.as_ptr(),
            swapchainCount: swapchains.len() as u32,
            pSwapchains: swapchains.as_ptr(),
            pImageIndices: image_indices.as_ptr(),
            pResults: if results.is_empty() {
                ptr::null_mut()
            } else {
                debug_assert!(results.len() == swapchains.len());
                results.as_mut_ptr()
            },
        }
    }
}
