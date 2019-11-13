use crate::load_function_ptrs;
use crate::prelude::*;

use std::os::raw::c_void;

load_function_ptrs!(InstanceWSIFunctions, {
    vkGetPhysicalDeviceSurfaceSupportKHR(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        surface: VkSurfaceKHR,
        pSupported: *mut VkBool32
    ) -> VkResult,

    vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
        physicalDevice: VkPhysicalDevice,
        surface: VkSurfaceKHR,
        pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR
    ) -> VkResult,

    vkGetPhysicalDeviceSurfaceFormatsKHR(
        physicalDevice: VkPhysicalDevice,
        surface: VkSurfaceKHR,
        pSurfaceFormatCount: *mut u32,
        pSurfaceFormats: *mut VkSurfaceFormatKHR
    ) -> VkResult,

    vkGetPhysicalDeviceSurfacePresentModesKHR(
        physicalDevice: VkPhysicalDevice,
        surface: VkSurfaceKHR,
        pPresentModeCount: *mut u32,
        pPresentModes: *mut VkPresentModeKHR
    ) -> VkResult,

    vkDestroySurfaceKHR(
        Instance: VkInstance,
        surface: VkSurfaceKHR,
        pAllocator: *const VkAllocationCallbacks
    ) -> (),

/*
    vkCreateXlibSurfaceKHR(
        Instance: VkInstance,
        pCreateInfo: *const VkXlibSurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR
    ) -> VkResult,

    vkCreateXcbSurfaceKHR(
        Instance: VkInstance,
        pCreateInfo: *const VkXcbSurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR
    ) -> VkResult,

    vkCreateWaylandSurfaceKHR(
        Instance: VkInstance,
        pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR
    ) -> VkResult,

    vkCreateMirSurfaceKHR(
        Instance: VkInstance,
        pCreateInfo: *const VkMirSurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR
    ) -> VkResult,

    vkCreateAndroidSurfaceKHR(
        Instance: VkInstance,
        pCreateInfo: *const VkAndroidSurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR
    ) -> VkResult,

    vkCreateWin32SurfaceKHR(
        Instance: VkInstance,
        pCreateInfo: *const VkWin32SurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR
    ) -> VkResult,

    vkCreateMacOSSurfaceMVK(
        Instance: VkInstance,
        pCreateInfo: *const VkMacOSSurfaceCreateInfoMVK,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR
    ) -> VkResult,

    vkGetPhysicalDeviceXlibPresentationSupportKHR(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        dpy: *mut c_void,
        visualID: u32
    ) -> VkBool32,

    vkGetPhysicalDeviceXcbPresentationSupportKHR(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        connection: *mut c_void,
        visual_id: u32
    ) -> VkBool32,

    vkGetPhysicalDeviceWaylandPresentationSupportKHR(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        display: *mut c_void
    ) -> VkBool32,

    vkGetPhysicalDeviceMirPresentationSupportKHR(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        connection: *mut c_void
    ) -> VkBool32,

    vkGetPhysicalDeviceWin32PresentationSupportKHR(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32
    ) -> VkBool32,

    vkGetPhysicalDeviceDisplayPropertiesKHR(
        physicalDevice: VkPhysicalDevice,
        pPropertyCount: *mut u32,
        pProperties: *mut VkDisplayPropertiesKHR
    ) -> VkResult,

    vkGetPhysicalDeviceDisplayPlanePropertiesKHR(
        physicalDevice: VkPhysicalDevice,
        pPropertyCount: *mut u32,
        pProperties: *mut VkDisplayPlanePropertiesKHR
    ) -> VkResult,

    vkGetDisplayPlaneSupportedDisplaysKHR(
        physicalDevice: VkPhysicalDevice,
        planeIndex: u32,
        pDisplayCount: *mut u32,
        pDisplays: *mut VkDisplayKHR
    ) -> VkResult,

    vkGetDisplayModePropertiesKHR(
        physicalDevice: VkPhysicalDevice,
        display: VkDisplayKHR,
        pPropertyCount: *mut u32,
        pProperties: *mut VkDisplayModePropertiesKHR
    ) -> VkResult,

    vkCreateDisplayModeKHR(
        physicalDevice: VkPhysicalDevice,
        display: VkDisplayKHR,
        pCreateInfo: *const VkDisplayModeCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pMode: *mut VkDisplayModeKHR
    ) -> VkResult,

    vkGetDisplayPlaneCapabilitiesKHR(
        physicalDevice: VkPhysicalDevice,
        mode: VkDisplayModeKHR,
        planeIndex: u32,
        pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR
    ) -> VkResult,

    vkCreateDisplayPlaneSurfaceKHR(
        Instance: VkInstance,
        pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR
    ) -> VkResult,
    */
});
