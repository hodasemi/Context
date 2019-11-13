use shared_library::dynamic_library::DynamicLibrary;
use utilities::prelude::*;

use crate::impl_vk_handle;
use crate::loader::*;
use crate::prelude::*;
use crate::Extensions;

use std::collections::HashSet;
use std::fmt;
use std::mem::MaybeUninit;
use std::ptr;
use std::sync::Arc;

use std::os::raw::c_char;
use std::os::raw::c_void;

use std::ffi::CStr;
use std::ffi::CString;

Extensions!(InstanceExtensions, {
    (xlib_surface, "VK_KHR_xlib_surface"),
    (wayland_surface, "VK_KHR_wayland_surface"),
    (android_surface, "VK_KHR_android_surface"),
    (macos_surface, "VK_KHR_macos_surface"),
    (win32_surface, "VK_KHR_win32_surface"),
    (surface, "VK_KHR_surface"),
    (physical_device_properties2, "VK_KHR_get_physical_device_properties2"),
});

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct VulkanDebugInfo {
    pub debugging: bool,
    pub steam_layer: bool,
    pub verbose: bool,
    pub renderdoc: bool,
    pub use_util: bool,
}

pub struct Instance {
    _vulkan_library: Option<DynamicLibrary>,

    _static_functions: StaticFunctions,
    _entry_functions: EntryFunctions,
    instance_functions: InstanceFunctions,
    instance_wsi_functions: InstanceWSIFunctions,
    physical_device_properties2_functions: PhysicalDeviceProperties2Functions,

    debug_report_callback_functions: DebugReportCallbackFunctions,
    debug_utils_messenger_functions: DebugUtilsMessengerFunctions,

    instance: VkInstance,

    instance_extensions: InstanceExtensions,

    debug_report: Option<VkDebugReportCallbackEXT>,
    debug_utils: Option<VkDebugUtilsMessengerEXT>,
}

struct Layer {
    props: Vec<VkLayerProperties>,
}

impl Layer {
    fn create(entry_functions: &EntryFunctions) -> VerboseResult<Layer> {
        Ok(Layer {
            props: Instance::enumerate_layer_properties(
                entry_functions.vkEnumerateInstanceLayerProperties,
            )?,
        })
    }

    fn names(
        &self,
        steam_layer: bool,
        verbose: bool,
        renderdoc: bool,
    ) -> VerboseResult<Vec<VkString>> {
        let mut names = Vec::new();

        for i in 0..self.props.len() {
            let name_string = self.props[i].layer_name()?;
            let name = name_string.as_str();

            if name == "VK_LAYER_LUNARG_core_validation"
                || name == "VK_LAYER_GOOGLE_threading"
                || name == "VK_LAYER_LUNARG_parameter_validation"
                || name == "VK_LAYER_LUNARG_object_tracker"
                || name == "VK_LAYER_LUNARG_standard_validation"
                || name == "VK_LAYER_GOOGLE_unique_objects"
                || name == "VK_LAYER_KHRONOS_validation"
            // || name == "VK_LAYER_LUNARG_monitor"
            {
                names.push(name_string.clone());
            }

            if verbose && name == "VK_LAYER_LUNARG_api_dump" {
                names.push(name_string.clone());
            }

            if renderdoc && name == "VK_LAYER_RENDERDOC_Capture" {
                names.push(name_string.clone());
            }

            if steam_layer
                && (name == "VK_LAYER_VALVE_steam_overlay_64"
                    || name == "VK_LAYER_VALVE_steam_overlay_32")
            {
                names.push(name_string.clone());
            }
        }

        Ok(names)
    }
}

impl Instance {
    pub fn new(
        app_info: VkApplicationInfo<'_>,
        debug_info: VulkanDebugInfo,
        extensions: InstanceExtensions,
    ) -> VerboseResult<Arc<Instance>> {
        let (library, static_functions) = load_static()?;
        let entry_functions = load_entry(&static_functions);

        let layers = if debug_info.debugging {
            let layer_object = Layer::create(&entry_functions)?;

            layer_object.names(
                debug_info.steam_layer,
                debug_info.verbose,
                debug_info.renderdoc,
            )?
        } else {
            Vec::new()
        };

        let mut checked_extensions = Vec::new();
        let mut extension_list = extensions.as_list();

        if debug_info.debugging {
            extension_list.push(VkString::new("VK_EXT_debug_report"));

            if debug_info.use_util {
                extension_list.push(VkString::new("VK_EXT_debug_utils"));
            }
        }

        if !extension_list.is_empty() {
            let extension_properties = Self::get_extension_properties(&entry_functions, &layers)?;

            for extension in extension_list {
                for ext_prop in &extension_properties {
                    if extension == *ext_prop {
                        checked_extensions.push(extension);
                        break;
                    }
                }
            }
        }

        // instance create info
        let layer_names = VkNames::new(layers.as_slice());
        let extension_names = VkNames::new(checked_extensions.as_slice());
        let instance_ci = VkInstanceCreateInfo::new(
            VK_INSTANCE_CREATE_NULL_BIT,
            &app_info,
            &layer_names,
            &extension_names,
        );

        if debug_info.debugging {
            println!("enabled layers ({}):", layer_names.len());

            for layer_name in layer_names.iter() {
                println!("\t- {:?}", layer_name);
            }

            println!("\nenabled instance extensions ({}):", extension_names.len());

            for extension_name in extension_names.iter() {
                println!("\t- {:?}", extension_name);
            }

            println!();
        }

        let enabled_extensions = InstanceExtensions::from_list(&checked_extensions);

        if debug_info.debugging {
            if let Err(missing_extensions) = extensions.check_availability(&enabled_extensions) {
                for m in missing_extensions {
                    println!("{}", m);
                }
            }
        }

        let instance = unsafe {
            let mut instance = MaybeUninit::uninit();
            let result =
                entry_functions.vkCreateInstance(&instance_ci, ptr::null(), instance.as_mut_ptr());

            if result == VK_SUCCESS {
                instance.assume_init()
            } else {
                create_error!(format!("failed creating VkInstance handle: {:?}", result))
            }
        };

        let instance_functions = load_instance(&static_functions, instance);
        let instance_wsi_functions = load_instance_wsi(&static_functions, instance);
        let physical_device_properties2_functions =
            load_physical_device_properties_2(&static_functions, instance);
        let debug_report_callback_functions = load_debug_report_ext(&static_functions, instance);
        let debug_utils_messenger_functions = load_debug_utils_ext(&static_functions, instance);

        let mut instance = Instance {
            _vulkan_library: Some(library),

            _static_functions: static_functions,
            _entry_functions: entry_functions,
            instance_functions,
            instance_wsi_functions,
            physical_device_properties2_functions,

            debug_report_callback_functions,
            debug_utils_messenger_functions,

            instance,

            instance_extensions: enabled_extensions,

            debug_utils: None,
            debug_report: None,
        };

        if !layers.is_empty() {
            let use_util = if debug_info.use_util {
                // TODO: debug util doesnt output anything
                match instance.create_debug_util() {
                    Ok(_) => true,
                    Err(msg) => {
                        println!("failed creating debug util: {}", msg);
                        false
                    }
                }
            } else {
                false
            };

            // only create debug report when debug util isnt created
            if !use_util {
                if let Err(msg) = instance.create_debug_report() {
                    println!("failed creating debug report: {}", msg);
                }
            }
        }

        Ok(Arc::new(instance))
    }

    pub fn vk_instance(&self) -> VkInstance {
        self.instance
    }

    pub fn enabled_extensions(&self) -> &InstanceExtensions {
        &self.instance_extensions
    }
}

unsafe impl Send for Instance {}
unsafe impl Sync for Instance {}

impl_vk_handle!(Instance, VkInstance, instance);

// private
impl Instance {
    fn create_debug_report(&mut self) -> VerboseResult<()> {
        let debug_report_info = VkDebugReportCallbackCreateInfoEXT::new(
            VK_DEBUG_REPORT_WARNING_BIT_EXT
                | VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT
                | VK_DEBUG_REPORT_ERROR_BIT_EXT,
            Instance::debug_report_callback,
        );

        let debug_report = self.create_debug_report_callbacks(&debug_report_info)?;

        self.debug_report = Some(debug_report);

        Ok(())
    }

    fn create_debug_util(&mut self) -> VerboseResult<()> {
        let debug_util_create_ci = VkDebugUtilsMessengerCreateInfoEXT::new(
            VK_DEBUG_UTILS_MESSENGER_CREATE_NULL_BIT,
            VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT
                | VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT,
            VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT
                | VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT
                | VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT,
            Self::debug_util_callback,
        );

        let debug_util_messenger = self.create_debug_utils_messenger(&debug_util_create_ci)?;

        self.debug_utils = Some(debug_util_messenger);

        Ok(())
    }

    fn get_extension_properties(
        entry_functions: &EntryFunctions,
        layers: &[VkString],
    ) -> VerboseResult<Vec<VkString>> {
        let mut properties = HashSet::new();

        let default_properties = Self::enumerate_extension_properties(
            entry_functions.vkEnumerateInstanceExtensionProperties,
            None,
        )?;

        for property in default_properties {
            let prop_string = VkString::new(&property.extension_name()?);

            properties.insert(prop_string);
        }

        for layer in layers {
            let tmp_properties = Self::enumerate_extension_properties(
                entry_functions.vkEnumerateInstanceExtensionProperties,
                Some(layer),
            )?;

            for property in tmp_properties {
                let prop_string = VkString::new(&property.extension_name()?);

                properties.insert(prop_string);
            }
        }

        Ok(properties.iter().cloned().collect())
    }
}

// debug
impl Instance {
    extern "system" fn debug_report_callback(
        flags: VkDebugReportFlagsEXT,
        object_type: VkDebugReportObjectTypeEXT,
        _src_object: u64,
        _location: usize,
        _msg_code: i32,
        _layer_prefix: *const c_char,
        msg: *const c_char,
        _user_data: *mut c_void,
    ) -> VkBool32 {
        let mut output: String = String::new();

        if (flags & VK_DEBUG_REPORT_INFORMATION_BIT_EXT) != 0 {
            output += "INFO: ";
        } else if (flags & VK_DEBUG_REPORT_WARNING_BIT_EXT) != 0 {
            output += "WARNING: ";
        } else if (flags & VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT) != 0 {
            output += "PERFORMANCE: ";
        } else if (flags & VK_DEBUG_REPORT_ERROR_BIT_EXT) != 0 {
            output += "ERROR: ";
        } else if (flags & VK_DEBUG_REPORT_DEBUG_BIT_EXT) != 0 {
            output += "DEBUG: ";
        }

        output += "OBJ( ";

        match object_type {
            VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT => output += "UNKNOWN",
            VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT => output += "INSTANCE",
            VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT => output += "PHYSICAL DEVICE",
            VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT => output += "DEVICE",
            VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT => output += "QUEUE",
            VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT => output += "SEMAPHORE",
            VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT => output += "COMMAND BUFFER",
            VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT => output += "FENCE",
            VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT => output += "DEVICE MEMORY",
            VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT => output += "BUFFER",
            VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT => output += "IMAGE",
            VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT => output += "EVENT",
            VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT => output += "QUERY POOL",
            VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT => output += "BUFFER VIEW",
            VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT => output += "IMAGE VIEW",
            VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT => output += "SHADER MODULE",
            VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT => output += "PIPELINE CACHE",
            VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT => output += "PIPELINE LAYOUT",
            VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT => output += "RENDER PASS",
            VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT => output += "PIPELINE",
            VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT => {
                output += "DESCRIPTOR SET LAYOUT"
            }
            VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT => output += "SAMPLER",
            VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT => output += "DESCRIPTOR POOL",
            VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT => output += "DESCRIPTOR SET",
            VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT => output += "FRAMEBUFFER",
            VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT => output += "COMMAND POOL",
            VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT => output += "SURFACE KHR",
            VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT => output += "SWAPCHAIN KHR",
            VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_EXT => output += "DEBUG REPORT",
        }

        let tmp1 = unsafe { CString::from_raw(msg as *mut c_char) };
        let tmp2 = tmp1.into_string().unwrap();

        output += " ):\n\t";
        output += tmp2.as_ref();

        println!("{}", output);

        VK_TRUE
    }

    extern "system" fn debug_util_callback(
        message_severity: VkDebugUtilsMessageSeverityFlagsEXT,
        message_type: VkDebugUtilsMessageTypeFlagsEXT,
        messenger_data: *const VkDebugUtilsMessengerCallbackDataEXT,
        _user_data: *mut c_void,
    ) -> VkBool32 {
        let mut output = String::new();

        if (message_severity & VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT) != 0 {
            output += "VERBOSE:";
        } else if (message_severity & VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT) != 0 {
            output += "INFO:";
        } else if (message_severity & VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT) != 0 {
            output += "WARNING:";
        } else if (message_severity & VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT) != 0 {
            output += "ERROR:";
        }

        output += "TYPE( ";

        if (message_type & VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT) != 0 {
            output += "GENERAL";
        } else if (message_type & VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT) != 0 {
            output += "VALIDATION";
        } else if (message_type & VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT) != 0 {
            output += "PERFORMANCE";
        }

        output += " )";

        let data = unsafe { &*messenger_data };
        let objects = data.objects();

        for object in objects {
            output += "OBJ( ";

            match object.objectType {
                VK_OBJECT_TYPE_UNKNOWN => output += "UNKNOWN",
                VK_OBJECT_TYPE_INSTANCE => output += "INSTANCE",
                VK_OBJECT_TYPE_PHYSICAL_DEVICE => output += "PHYSICAL DEVICE",
                VK_OBJECT_TYPE_DEVICE => output += "DEVICE",
                VK_OBJECT_TYPE_QUEUE => output += "QUEUE",
                VK_OBJECT_TYPE_SEMAPHORE => output += "SEMAPHORE",
                VK_OBJECT_TYPE_COMMAND_BUFFER => output += "COMMAND BUFFER",
                VK_OBJECT_TYPE_FENCE => output += "FENCE",
                VK_OBJECT_TYPE_DEVICE_MEMORY => output += "DEVICE MEMORY",
                VK_OBJECT_TYPE_BUFFER => output += "BUFFER",
                VK_OBJECT_TYPE_IMAGE => output += "IMAGE",
                VK_OBJECT_TYPE_EVENT => output += "EVENT",
                VK_OBJECT_TYPE_QUERY_POOL => output += "QUERY POOL",
                VK_OBJECT_TYPE_BUFFER_VIEW => output += "BUFFER VIEW",
                VK_OBJECT_TYPE_IMAGE_VIEW => output += "IMAGE VIEW",
                VK_OBJECT_TYPE_SHADER_MODULE => output += "SHADER MODULE",
                VK_OBJECT_TYPE_PIPELINE_CACHE => output += "PIPELINE CACHE",
                VK_OBJECT_TYPE_PIPELINE_LAYOUT => output += "PIPELINE LAYOUT",
                VK_OBJECT_TYPE_RENDER_PASS => output += "RENDER PASS",
                VK_OBJECT_TYPE_PIPELINE => output += "PIPELINE",
                VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT => output += "DESCRIPTOR SET LAYOUT",
                VK_OBJECT_TYPE_SAMPLER => output += "SAMPLER",
                VK_OBJECT_TYPE_DESCRIPTOR_POOL => output += "DESCRIPTOR POOL",
                VK_OBJECT_TYPE_DESCRIPTOR_SET => output += "DESCRIPTOR SET",
                VK_OBJECT_TYPE_FRAMEBUFFER => output += "FRAMEBUFFER",
                VK_OBJECT_TYPE_COMMAND_POOL => output += "COMMAND POOL",
                VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION => output += "SAMPLER YCBCR CONVERSION",
                VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE => output += "DESCRIPTOR UPDATE TEMPLATE",
                VK_OBJECT_TYPE_SURFACE_KHR => output += "SURFACE KHR",
                VK_OBJECT_TYPE_SWAPCHAIN_KHR => output += "SWAPCHAIN KHR",
                VK_OBJECT_TYPE_DISPLAY_KHR => output += "DISPLAY KHR",
                VK_OBJECT_TYPE_DISPLAY_MODE_KHR => output += "DISPLAY MODE KHR",
                VK_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT => output += "DEBUG REPORT CALLBACK EXT",
                VK_OBJECT_TYPE_OBJECT_TABLE_NVX => output += "OBJECT TABLE NVX",
                VK_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_NVX => {
                    output += "INDIRECT COMMANDS LAYOUT NVX"
                }
                VK_OBJECT_TYPE_VALIDATION_CACHE_EXT => output += "VALIDATION CACHE EXT",
                VK_OBJECT_TYPE_DEBUG_UTILS_MESSENGER_EXT => output += "DEBUG UTILS MESSENGER EXT",
            }

            output += " ) ";
        }

        output += ": ";

        if let Ok(message) = data.message() {
            output += &message;
        }

        VK_TRUE
    }
}

impl fmt::Debug for Instance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Instance (VkInstance: {:#?})", self.instance)
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        if let Some(debug_report) = &self.debug_report {
            self.destroy_debug_report_callbacks(*debug_report);
        }

        if let Some(debug_utils) = &self.debug_utils {
            self.destroy_debug_utils_messenger(*debug_utils);
        }

        self.destroy_instance();
    }
}

// private wrapper
impl Instance {
    fn enumerate_layer_properties(
        enumerate_instance_layer_properties: PFN_vkEnumerateInstanceLayerProperties,
    ) -> VerboseResult<Vec<VkLayerProperties>> {
        let mut property_count: u32 = 0;

        // get the amount of properties
        let result = enumerate_instance_layer_properties(&mut property_count, ptr::null_mut());

        if result != VK_SUCCESS {
            create_error!(format!("failed enumerating instance layers: {:?}", result));
        }

        let mut properties = Vec::with_capacity(property_count as usize);
        unsafe { properties.set_len(property_count as usize) };

        // get the properties
        let result =
            enumerate_instance_layer_properties(&mut property_count, properties.as_mut_ptr());

        if result == VK_SUCCESS {
            Ok(properties)
        } else {
            create_error!(format!("failed enumerating instance layers: {:?}", result));
        }
    }

    fn enumerate_extension_properties(
        enumerate_instance_extension_properties: PFN_vkEnumerateInstanceExtensionProperties,
        layer_name: Option<&VkString>,
    ) -> VerboseResult<Vec<VkExtensionProperties>> {
        let mut count = 0;
        let name = match layer_name {
            Some(name) => name.as_ptr(),
            None => ptr::null(),
        };

        let mut result = enumerate_instance_extension_properties(name, &mut count, ptr::null_mut());

        if result != VK_SUCCESS {
            create_error!(format!(
                "failed enumerating instance extensions: {:?}",
                result
            ));
        }

        let mut properties = Vec::with_capacity(count as usize);
        unsafe { properties.set_len(count as usize) };

        result = enumerate_instance_extension_properties(name, &mut count, properties.as_mut_ptr());

        if result == VK_SUCCESS {
            Ok(properties)
        } else {
            create_error!(format!(
                "failed enumerating instance extensions: {:?}",
                result
            ));
        }
    }

    fn destroy_instance(&self) {
        unsafe {
            self.instance_functions
                .vkDestroyInstance(self.instance, ptr::null());
        }
    }
}

// public, wrapped vulkan calls
impl Instance {
    pub fn create_debug_report_callbacks(
        &self,
        debug_report_callback_create_info: &VkDebugReportCallbackCreateInfoEXT,
    ) -> VerboseResult<VkDebugReportCallbackEXT> {
        unsafe {
            let mut debug_report_callback = MaybeUninit::uninit();

            let result = self
                .debug_report_callback_functions
                .vkCreateDebugReportCallbackEXT(
                    self.instance,
                    debug_report_callback_create_info,
                    ptr::null(),
                    debug_report_callback.as_mut_ptr(),
                );

            if result == VK_SUCCESS {
                Ok(debug_report_callback.assume_init())
            } else {
                create_error!(format!(
                    "failed creating VkDebugReportCallbackEXT: {:?}",
                    result
                ));
            }
        }
    }

    pub fn destroy_debug_report_callbacks(&self, debug_report_callback: VkDebugReportCallbackEXT) {
        unsafe {
            self.debug_report_callback_functions
                .vkDestroyDebugReportCallbackEXT(self.instance, debug_report_callback, ptr::null())
        }
    }

    pub fn create_debug_utils_messenger(
        &self,
        debug_utils_messenger_create_info: &VkDebugUtilsMessengerCreateInfoEXT,
    ) -> VerboseResult<VkDebugUtilsMessengerEXT> {
        unsafe {
            let mut debug_utils_messenger = MaybeUninit::uninit();

            let result = self
                .debug_utils_messenger_functions
                .vkCreateDebugUtilsMessengerEXT(
                    self.instance,
                    debug_utils_messenger_create_info,
                    ptr::null(),
                    debug_utils_messenger.as_mut_ptr(),
                );

            if result == VK_SUCCESS {
                Ok(debug_utils_messenger.assume_init())
            } else {
                create_error!(format!(
                    "failed creating VkDebugUtilsMessengerEXT: {:?}",
                    result
                ));
            }
        }
    }

    pub fn destroy_debug_utils_messenger(&self, debug_utils_messenger: VkDebugUtilsMessengerEXT) {
        unsafe {
            self.debug_utils_messenger_functions
                .vkDestroyDebugUtilsMessengerEXT(self.instance, debug_utils_messenger, ptr::null())
        }
    }

    pub fn get_device_proc_addr(&self, device: VkDevice, name: VkString) -> PFN_vkVoidFunction {
        unsafe {
            self.instance_functions
                .vkGetDeviceProcAddr(device, name.as_ptr())
        }
    }

    pub fn get_device_proc_addr_raw(&self, device: VkDevice, name: &CStr) -> PFN_vkVoidFunction {
        unsafe {
            self.instance_functions
                .vkGetDeviceProcAddr(device, name.as_ptr())
        }
    }

    pub fn enumerate_physical_devices(&self) -> VerboseResult<Vec<VkPhysicalDevice>> {
        let mut count = 0;

        let result = unsafe {
            self.instance_functions.vkEnumeratePhysicalDevices(
                self.instance,
                &mut count,
                ptr::null_mut(),
            )
        };

        if result != VK_SUCCESS {
            create_error!(format!("failed enumerating physical devices {:?}", result));
        }

        let mut physical_devices = Vec::with_capacity(count as usize);
        unsafe { physical_devices.set_len(count as usize) };

        let result = unsafe {
            self.instance_functions.vkEnumeratePhysicalDevices(
                self.instance,
                &mut count,
                physical_devices.as_mut_ptr(),
            )
        };

        if result == VK_SUCCESS {
            Ok(physical_devices)
        } else {
            create_error!(format!("failed enumerating physical devices {:?}", result));
        }
    }

    pub fn physical_device_properties(
        &self,
        physical_device: VkPhysicalDevice,
    ) -> VkPhysicalDeviceProperties {
        unsafe {
            let mut physical_device_properties = MaybeUninit::uninit();

            self.instance_functions.vkGetPhysicalDeviceProperties(
                physical_device,
                physical_device_properties.as_mut_ptr(),
            );

            physical_device_properties.assume_init()
        }
    }

    pub fn physical_device_features(
        &self,
        physical_device: VkPhysicalDevice,
    ) -> VkPhysicalDeviceFeatures {
        unsafe {
            let mut physical_device_features = MaybeUninit::uninit();

            self.instance_functions.vkGetPhysicalDeviceFeatures(
                physical_device,
                physical_device_features.as_mut_ptr(),
            );

            physical_device_features.assume_init()
        }
    }

    pub fn physical_device_format_properties(
        &self,
        physical_device: VkPhysicalDevice,
        format: VkFormat,
    ) -> VkFormatProperties {
        unsafe {
            let mut physical_device_format_properties = MaybeUninit::uninit();

            self.instance_functions.vkGetPhysicalDeviceFormatProperties(
                physical_device,
                format,
                physical_device_format_properties.as_mut_ptr(),
            );

            physical_device_format_properties.assume_init()
        }
    }

    pub fn physical_device_queue_family_properties(
        &self,
        physical_device: VkPhysicalDevice,
    ) -> Vec<VkQueueFamilyProperties> {
        let mut count = 0;

        unsafe {
            self.instance_functions
                .vkGetPhysicalDeviceQueueFamilyProperties(
                    physical_device,
                    &mut count,
                    ptr::null_mut(),
                );
        }

        let mut queue_family_properties = Vec::with_capacity(count as usize);
        unsafe { queue_family_properties.set_len(count as usize) };

        unsafe {
            self.instance_functions
                .vkGetPhysicalDeviceQueueFamilyProperties(
                    physical_device,
                    &mut count,
                    queue_family_properties.as_mut_ptr(),
                );
        }

        queue_family_properties
    }

    pub fn physical_device_memory_properties(
        &self,
        physical_device: VkPhysicalDevice,
    ) -> VkPhysicalDeviceMemoryProperties {
        unsafe {
            let mut physical_device_memory_properties = MaybeUninit::uninit();

            self.instance_functions.vkGetPhysicalDeviceMemoryProperties(
                physical_device,
                physical_device_memory_properties.as_mut_ptr(),
            );

            physical_device_memory_properties.assume_init()
        }
    }

    pub fn physical_device_sparse_image_format_properties<T>(
        &self,
        physical_device: VkPhysicalDevice,
        format: VkFormat,
        ty: VkImageType,
        samples: VkSampleCountFlags,
        usage: impl Into<VkImageUsageFlagBits>,
        tiling: VkImageTiling,
    ) -> Vec<VkSparseImageFormatProperties> {
        let mut count = 0;
        let usage = usage.into();

        unsafe {
            self.instance_functions
                .vkGetPhysicalDeviceSparseImageFormatProperties(
                    physical_device,
                    format,
                    ty,
                    samples,
                    usage,
                    tiling,
                    &mut count,
                    ptr::null_mut(),
                );
        }

        let mut sparse_image_formats = Vec::with_capacity(count as usize);
        unsafe { sparse_image_formats.set_len(count as usize) };

        unsafe {
            self.instance_functions
                .vkGetPhysicalDeviceSparseImageFormatProperties(
                    physical_device,
                    format,
                    ty,
                    samples,
                    usage,
                    tiling,
                    &mut count,
                    sparse_image_formats.as_mut_ptr(),
                );
        }

        sparse_image_formats
    }

    pub fn physical_device_image_format_properties(
        &self,
        physical_device: VkPhysicalDevice,
        format: VkFormat,
        image_type: VkImageType,
        tiling: VkImageTiling,
        usage: impl Into<VkImageUsageFlagBits>,
        flags: impl Into<VkImageCreateFlagBits>,
    ) -> VerboseResult<VkImageFormatProperties> {
        unsafe {
            let mut image_format_properties = MaybeUninit::uninit();

            let result = self
                .instance_functions
                .vkGetPhysicalDeviceImageFormatProperties(
                    physical_device,
                    format,
                    image_type,
                    tiling,
                    usage.into(),
                    flags.into(),
                    image_format_properties.as_mut_ptr(),
                );

            if result == VK_SUCCESS {
                Ok(image_format_properties.assume_init())
            } else {
                create_error!(format!(
                    "failed getting physical device image format properties {:?}",
                    result
                ))
            }
        }
    }

    pub fn create_device<'a>(
        &self,
        physical_device: VkPhysicalDevice,
        device_create_info: &'a VkDeviceCreateInfo<'a>,
    ) -> VerboseResult<VkDevice> {
        unsafe {
            let mut device = MaybeUninit::uninit();

            let result = self.instance_functions.vkCreateDevice(
                physical_device,
                device_create_info,
                ptr::null(),
                device.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(device.assume_init())
            } else {
                create_error!(format!("failed creating VkDevice {:?}", result))
            }
        }
    }

    pub fn physical_device_surface_support(
        &self,
        physical_device: VkPhysicalDevice,
        queue_family_index: u32,
        surface: VkSurfaceKHR,
    ) -> VerboseResult<bool> {
        unsafe {
            let mut supported = MaybeUninit::uninit();

            let result = self
                .instance_wsi_functions
                .vkGetPhysicalDeviceSurfaceSupportKHR(
                    physical_device,
                    queue_family_index,
                    surface,
                    supported.as_mut_ptr(),
                );

            if result == VK_SUCCESS {
                Ok(supported.assume_init() == VK_TRUE)
            } else {
                create_error!(format!("failed getting surface support {:?}", result))
            }
        }
    }

    pub fn physical_device_surface_capabilities(
        &self,
        physical_device: VkPhysicalDevice,
        surface: VkSurfaceKHR,
    ) -> VerboseResult<VkSurfaceCapabilitiesKHR> {
        unsafe {
            let mut surface_capabilities = MaybeUninit::uninit();

            let result = self
                .instance_wsi_functions
                .vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
                    physical_device,
                    surface,
                    surface_capabilities.as_mut_ptr(),
                );

            if result == VK_SUCCESS {
                Ok(surface_capabilities.assume_init())
            } else {
                create_error!(format!("failed getting surface capabilities {:?}", result))
            }
        }
    }

    pub fn physical_device_surface_formats(
        &self,
        physical_device: VkPhysicalDevice,
        surface: VkSurfaceKHR,
    ) -> VerboseResult<Vec<VkSurfaceFormatKHR>> {
        let mut count = 0;

        let result = unsafe {
            self.instance_wsi_functions
                .vkGetPhysicalDeviceSurfaceFormatsKHR(
                    physical_device,
                    surface,
                    &mut count,
                    ptr::null_mut(),
                )
        };

        if result != VK_SUCCESS {
            create_error!(format!("failed getting surface formats {:?}", result))
        }

        let mut surface_formats = Vec::with_capacity(count as usize);
        unsafe { surface_formats.set_len(count as usize) };

        let result = unsafe {
            self.instance_wsi_functions
                .vkGetPhysicalDeviceSurfaceFormatsKHR(
                    physical_device,
                    surface,
                    &mut count,
                    surface_formats.as_mut_ptr(),
                )
        };

        if result == VK_SUCCESS {
            Ok(surface_formats)
        } else {
            create_error!(format!("failed getting surface formats {:?}", result))
        }
    }

    pub fn physical_device_present_modes(
        &self,
        physical_device: VkPhysicalDevice,
        surface: VkSurfaceKHR,
    ) -> VerboseResult<Vec<VkPresentModeKHR>> {
        let mut count = 0;

        let result = unsafe {
            self.instance_wsi_functions
                .vkGetPhysicalDeviceSurfacePresentModesKHR(
                    physical_device,
                    surface,
                    &mut count,
                    ptr::null_mut(),
                )
        };

        if result != VK_SUCCESS {
            create_error!(format!("failed getting present modes {:?}", result))
        }

        let mut surface_present_modes = Vec::with_capacity(count as usize);
        unsafe { surface_present_modes.set_len(count as usize) };

        let result = unsafe {
            self.instance_wsi_functions
                .vkGetPhysicalDeviceSurfacePresentModesKHR(
                    physical_device,
                    surface,
                    &mut count,
                    surface_present_modes.as_mut_ptr(),
                )
        };

        if result == VK_SUCCESS {
            Ok(surface_present_modes)
        } else {
            create_error!(format!("failed getting present modes {:?}", result))
        }
    }

    pub fn enumerate_device_extensions(
        &self,
        physical_device: VkPhysicalDevice,
    ) -> VerboseResult<Vec<VkExtensionProperties>> {
        let mut count = 0;

        let result = unsafe {
            self.instance_functions
                .vkEnumerateDeviceExtensionProperties(
                    physical_device,
                    ptr::null(),
                    &mut count,
                    ptr::null_mut(),
                )
        };

        if result != VK_SUCCESS {
            create_error!(format!("failed enumerating device extensions {:?}", result))
        }

        let mut extension_properties = Vec::with_capacity(count as usize);
        unsafe { extension_properties.set_len(count as usize) };

        let result = unsafe {
            self.instance_functions
                .vkEnumerateDeviceExtensionProperties(
                    physical_device,
                    ptr::null(),
                    &mut count,
                    extension_properties.as_mut_ptr(),
                )
        };

        if result == VK_SUCCESS {
            Ok(extension_properties)
        } else {
            create_error!(format!("failed enumerating device extensions {:?}", result))
        }
    }

    pub fn physical_device_properties2(
        &self,
        physical_device: VkPhysicalDevice,
        device_properties: &mut VkPhysicalDeviceProperties2KHR,
    ) {
        unsafe {
            self.physical_device_properties2_functions
                .vkGetPhysicalDeviceProperties2KHR(physical_device, device_properties);
        }
    }

    pub fn physical_device_features2(
        &self,
        physical_device: VkPhysicalDevice,
        device_features: &mut VkPhysicalDeviceFeatures2KHR,
    ) {
        unsafe {
            self.physical_device_properties2_functions
                .vkGetPhysicalDeviceFeatures2KHR(physical_device, device_features);
        }
    }

    pub fn physical_device_format_properties2(
        &self,
        physical_device: VkPhysicalDevice,
    ) -> VkFormatProperties2KHR<'_> {
        unsafe {
            let mut handle = MaybeUninit::uninit();

            self.physical_device_properties2_functions
                .vkGetPhysicalDeviceFormatProperties2KHR(physical_device, handle.as_mut_ptr());

            handle.assume_init()
        }
    }

    pub fn physical_device_image_format_properties2(
        &self,
        physical_device: VkPhysicalDevice,
        image_format_info: &VkPhysicalDeviceImageFormatInfo2KHR,
    ) -> VkImageFormatProperties2KHR<'_> {
        unsafe {
            let mut handle = MaybeUninit::uninit();

            self.physical_device_properties2_functions
                .vkGetPhysicalDeviceImageFormatProperties2KHR(
                    physical_device,
                    image_format_info,
                    handle.as_mut_ptr(),
                );

            handle.assume_init()
        }
    }

    pub fn physical_device_queue_family_properties2(
        &self,
        physical_device: VkPhysicalDevice,
    ) -> Vec<VkQueueFamilyProperties2KHR> {
        let mut count = 0;

        unsafe {
            self.physical_device_properties2_functions
                .vkGetPhysicalDeviceQueueFamilyProperties2KHR(
                    physical_device,
                    &mut count,
                    ptr::null_mut(),
                )
        };

        let mut family_queue_properties = Vec::with_capacity(count as usize);
        unsafe { family_queue_properties.set_len(count as usize) };

        unsafe {
            self.physical_device_properties2_functions
                .vkGetPhysicalDeviceQueueFamilyProperties2KHR(
                    physical_device,
                    &mut count,
                    family_queue_properties.as_mut_ptr(),
                )
        };

        family_queue_properties
    }

    pub fn physical_device_memory_properties2(
        &self,
        physical_device: VkPhysicalDevice,
    ) -> VkPhysicalDeviceMemoryProperties2KHR {
        unsafe {
            let mut handle = MaybeUninit::uninit();

            self.physical_device_properties2_functions
                .vkGetPhysicalDeviceMemoryProperties2KHR(physical_device, handle.as_mut_ptr());

            handle.assume_init()
        }
    }

    pub fn physical_device_memory_budget(
        &self,
        physical_device: VkPhysicalDevice,
    ) -> (VkPhysicalDeviceMemoryBudgetPropertiesEXT, u32) {
        unsafe {
            let mut properties = VkPhysicalDeviceMemoryProperties2KHR::default();
            let memory_budget = VkPhysicalDeviceMemoryBudgetPropertiesEXT::default();
            properties.chain(&memory_budget);

            self.physical_device_properties2_functions
                .vkGetPhysicalDeviceMemoryProperties2KHR(physical_device, &mut properties);

            (memory_budget, properties.memoryProperties.memoryHeapCount)
        }
    }

    pub fn physical_device_sparse_image_format_properties2(
        &self,
        physical_device: VkPhysicalDevice,
        format_info: &VkPhysicalDeviceSparseImageFormatInfo2KHR,
    ) -> Vec<VkSparseImageFormatProperties2KHR> {
        let mut count = 0;

        unsafe {
            self.physical_device_properties2_functions
                .vkGetPhysicalDeviceSparseImageFormatProperties2KHR(
                    physical_device,
                    format_info,
                    &mut count,
                    ptr::null_mut(),
                )
        };

        let mut sparse_image_formats = Vec::with_capacity(count as usize);
        unsafe { sparse_image_formats.set_len(count as usize) };

        unsafe {
            self.physical_device_properties2_functions
                .vkGetPhysicalDeviceSparseImageFormatProperties2KHR(
                    physical_device,
                    format_info,
                    &mut count,
                    sparse_image_formats.as_mut_ptr(),
                )
        };

        sparse_image_formats
    }

    pub fn destroy_surface(&self, surface: VkSurfaceKHR) {
        unsafe {
            self.instance_wsi_functions
                .vkDestroySurfaceKHR(self.instance, surface, ptr::null())
        };
    }
}
