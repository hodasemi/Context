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
pub struct VkSwapchainCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkSwapchainCreateFlagBitsKHR,
    pub surface: VkSurfaceKHR,
    pub minImageCount: u32,
    pub imageFormat: VkFormat,
    pub imageColorSpace: VkColorSpaceKHR,
    pub imageExtent: VkExtent2D,
    pub imageArrayLayers: u32,
    pub imageUsage: VkImageUsageFlagBits,
    pub imageSharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
    pub preTransform: VkSurfaceTransformFlagBitsKHR,
    pub compositeAlpha: VkCompositeAlphaFlagBitsKHR,
    pub presentMode: VkPresentModeKHR,
    pub clipped: VkBool32,
    pub oldSwapchain: VkSwapchainKHR,
}

impl VkSwapchainCreateInfoKHR {
    pub fn new<S, T, U, V, W>(
        flags: T,
        surface: VkSurfaceKHR,
        min_image_count: u32,
        format: VkFormat,
        color_space: VkColorSpaceKHR,
        extent: VkExtent2D,
        array_layers: u32,
        usage: U,
        sharing_mode: VkSharingMode,
        queue_family_indices: &[u32],
        pre_transform: V,
        composite_alpha: W,
        present_mode: VkPresentModeKHR,
        clipped: S,
    ) -> Self
    where
        T: Into<VkSwapchainCreateFlagBitsKHR>,
        U: Into<VkImageUsageFlagBits>,
        V: Into<VkSurfaceTransformFlagBitsKHR>,
        W: Into<VkCompositeAlphaFlagBitsKHR>,
        S: Into<VkBool32>,
    {
        VkSwapchainCreateInfoKHR {
            sType: VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
            pNext: ptr::null(),
            flags: flags.into(),
            surface,
            minImageCount: min_image_count,
            imageFormat: format,
            imageColorSpace: color_space,
            imageExtent: extent,
            imageArrayLayers: array_layers,
            imageUsage: usage.into(),
            imageSharingMode: sharing_mode,
            queueFamilyIndexCount: queue_family_indices.len() as u32,
            pQueueFamilyIndices: queue_family_indices.as_ptr(),
            preTransform: pre_transform.into(),
            compositeAlpha: composite_alpha.into(),
            presentMode: present_mode,
            clipped: clipped.into(),
            oldSwapchain: VkSwapchainKHR::NULL_HANDLE,
        }
    }

    pub fn set_old_swapchain(&mut self, swapchain: VkSwapchainKHR) {
        self.oldSwapchain = swapchain;
    }
}
