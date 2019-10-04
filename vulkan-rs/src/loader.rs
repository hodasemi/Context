use shared_library::dynamic_library::DynamicLibrary;

use utilities::prelude::*;

use crate::prelude::*;

use std::ffi::CStr;
use std::mem::transmute;
use std::path::Path;

#[cfg(target_os = "linux")]
const VULKAN_LIB: &str = "libvulkan.so";

#[cfg(target_os = "macos")]
const VULKAN_LIB: &str = "libMoltenVK.dylib";

#[cfg(target_os = "windows")]
const VULKAN_LIB: &str = "vulkan-1.dll";

pub fn load_static() -> VerboseResult<(DynamicLibrary, StaticFunctions)> {
    let lib = match DynamicLibrary::open(Some(Path::new(VULKAN_LIB))) {
        Ok(lib) => lib,
        Err(_) => create_error!(format!("failed loading {}", VULKAN_LIB)),
    };

    let static_functions = StaticFunctions::load(|name| unsafe {
        let str_name = name.to_str().expect("can't convert CStr");
        lib.symbol(str_name)
            .expect(&format!("failed getting {}", str_name))
    });

    Ok((lib, static_functions))
}

pub fn load_entry(static_functions: &StaticFunctions) -> EntryFunctions {
    EntryFunctions::load(|name| unsafe {
        transmute(static_functions.vkGetInstanceProcAddr(VkInstance::NULL_HANDLE, name.as_ptr()))
    })
}

pub fn load_instance(
    static_functions: &StaticFunctions,
    instance: VkInstance,
) -> InstanceFunctions {
    InstanceFunctions::load(|name| unsafe {
        transmute(static_functions.vkGetInstanceProcAddr(instance, name.as_ptr()))
    })
}

pub fn load_device<F>(f: F, device: VkDevice) -> DeviceFunctions
where
    F: Fn(VkDevice, &CStr) -> PFN_vkVoidFunction,
{
    DeviceFunctions::load(|name| unsafe { transmute(f(device, name)) })
}

pub fn load_debug_report_ext(
    static_functions: &StaticFunctions,
    instance: VkInstance,
) -> DebugReportCallbackFunctions {
    DebugReportCallbackFunctions::load(|name| unsafe {
        transmute(static_functions.vkGetInstanceProcAddr(instance, name.as_ptr()))
    })
}

pub fn load_debug_utils_ext(
    static_functions: &StaticFunctions,
    instance: VkInstance,
) -> DebugUtilsMessengerFunctions {
    DebugUtilsMessengerFunctions::load(|name| unsafe {
        transmute(static_functions.vkGetInstanceProcAddr(instance, name.as_ptr()))
    })
}

pub fn load_instance_wsi(
    static_functions: &StaticFunctions,
    instance: VkInstance,
) -> InstanceWSIFunctions {
    InstanceWSIFunctions::load(|name| unsafe {
        transmute(static_functions.vkGetInstanceProcAddr(instance, name.as_ptr()))
    })
}

pub fn load_device_wsi<F>(f: F, device: VkDevice) -> DeviceWSIFunctions
where
    F: Fn(VkDevice, &CStr) -> PFN_vkVoidFunction,
{
    DeviceWSIFunctions::load(|name| unsafe { transmute(f(device, name)) })
}

pub fn load_physical_device_properties_2(
    static_functions: &StaticFunctions,
    instance: VkInstance,
) -> PhysicalDeviceProperties2Functions {
    PhysicalDeviceProperties2Functions::load(|name| unsafe {
        transmute(static_functions.vkGetInstanceProcAddr(instance, name.as_ptr()))
    })
}

pub fn load_nv_ray_tracing<F>(f: F, device: VkDevice) -> NVRayTracingFunctions
where
    F: Fn(VkDevice, &CStr) -> PFN_vkVoidFunction,
{
    NVRayTracingFunctions::load(|name| unsafe { transmute(f(device, name)) })
}

pub fn load_maintenance3<F>(f: F, device: VkDevice) -> Maintenance3Functions
where
    F: Fn(VkDevice, &CStr) -> PFN_vkVoidFunction,
{
    Maintenance3Functions::load(|name| unsafe { transmute(f(device, name)) })
}
