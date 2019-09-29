#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use utilities::prelude::*;

use crate::prelude::*;

use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::sync::Arc;

const google_display_timing_name: &str = "VK_GOOGLE_display_timing";

#[repr(C)]
pub struct VkRefreshCycleDurationGOOGLE {
    refreshDuration: u64,
}

#[repr(C)]
#[derive(Clone)]
pub struct VkPastPresentationTimingGOOGLE {
    presentID: u32,
    desiredPresentTime: u64,
    actualPresentTime: u64,
    earliestPresentTime: u64,
    presentMargin: u64,
}

#[repr(C)]
pub struct VkPresentTimesInfoGOOGLE {
    sType: VkStructureType,
    pNext: *const c_void,
    swapchainCount: u32,
    pTimes: *const VkPresentTimeGOOGLE,
}

#[repr(C)]
pub struct VkPresentTimeGOOGLE {
    presentID: u32,
    desiredPresentTime: u32,
}

type PFN_vkGetPastPresentationTimingGOOGLE = extern "system" fn(
    VkDevice,
    VkSwapchainKHR,
    *mut u32,
    *mut VkPastPresentationTimingGOOGLE,
) -> VkResult;

type PFN_vkGetRefreshCycleDurationGOOGLE =
    extern "system" fn(VkDevice, VkSwapchainKHR, *mut VkRefreshCycleDurationGOOGLE) -> VkResult;

pub struct DisplayTiming {
    device: Arc<Device>,

    vkGetPastPresentationTimingGOOGLE: PFN_vkGetPastPresentationTimingGOOGLE,
    vkGetRefreshCycleDurationGOOGLE: PFN_vkGetRefreshCycleDurationGOOGLE,
}

impl DisplayTiming {
    pub fn new(device: Arc<Device>) -> VerboseResult<DisplayTiming> {
        // check for presence of VK_GOOGLE_display_timing extension
        if !device
            .physical_device()
            .extensions()
            .contains(&VkString::new(google_display_timing_name))
        {
            create_error!(format!(
                "Your device does not support the {} extension",
                google_display_timing_name
            ));
        }

        // get function pointers
        let opt_past_presentation_timing: Option<PFN_vkGetPastPresentationTimingGOOGLE> = unsafe {
            Some(mem::transmute(device.device_proc_addr(VkString::new(
                "vkGetPastPresentationTimingGOOGLE",
            ))))
        };

        let past_presentation_timing = match opt_past_presentation_timing {
            Some(pfn) => pfn,
            None => {
                create_error!("failed getting PFN_vkGetPastPresentationTimingGOOGLE".to_string())
            }
        };

        let opt_refresh_cycle_duration: Option<PFN_vkGetRefreshCycleDurationGOOGLE> = unsafe {
            Some(mem::transmute(device.device_proc_addr(VkString::new(
                "PFN_vkGetRefreshCycleDurationGOOGLE",
            ))))
        };

        let refresh_cycle_duration = match opt_refresh_cycle_duration {
            Some(pfn) => pfn,
            None => create_error!("failed getting PFN_vkGetRefreshCycleDurationGOOGLE".to_string()),
        };

        Ok(DisplayTiming {
            device,
            vkGetPastPresentationTimingGOOGLE: past_presentation_timing,
            vkGetRefreshCycleDurationGOOGLE: refresh_cycle_duration,
        })
    }

    pub fn past_presentation_timing(
        &self,
        swapchain: &Arc<Swapchain>,
    ) -> VerboseResult<Vec<VkPastPresentationTimingGOOGLE>> {
        let mut count = 0;

        let result = (self.vkGetPastPresentationTimingGOOGLE)(
            self.device.vk_handle(),
            swapchain.vk_handle(),
            &mut count,
            ptr::null_mut(),
        );

        if result != VK_SUCCESS {
            create_error!(format!("failed getting presentation timing count"));
        }

        let mut presentation_timings = vec![
            VkPastPresentationTimingGOOGLE {
                presentID: 0,
                desiredPresentTime: 0,
                actualPresentTime: 0,
                earliestPresentTime: 0,
                presentMargin: 0,
            };
            count as usize
        ];

        let result = (self.vkGetPastPresentationTimingGOOGLE)(
            self.device.vk_handle(),
            swapchain.vk_handle(),
            &mut count,
            presentation_timings.as_mut_ptr(),
        );

        if result != VK_SUCCESS {
            create_error!(format!("failed getting presentation timings"));
        }

        Ok(presentation_timings)
    }

    pub fn refresh_cycle_duration(&self, swapchain: &Arc<Swapchain>) -> VerboseResult<u64> {
        let mut refresh_cycle = VkRefreshCycleDurationGOOGLE { refreshDuration: 0 };

        let result = (self.vkGetRefreshCycleDurationGOOGLE)(
            self.device.vk_handle(),
            swapchain.vk_handle(),
            &mut refresh_cycle,
        );

        if result != VK_SUCCESS {
            create_error!(format!("failed getting refresh cycle duration"));
        }

        Ok(refresh_cycle.refreshDuration)
    }
}
