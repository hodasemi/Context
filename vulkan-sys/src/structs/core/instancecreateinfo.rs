use crate::prelude::*;

use std::ffi::CStr;
use std::fmt;
use std::marker::PhantomData;
use std::os::raw::{c_char, c_void};
use std::ptr;
use std::slice;

#[repr(C)]
pub struct VkInstanceCreateInfo<'a> {
    lt: PhantomData<&'a ()>,
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkInstanceCreateFlagBits,
    pub pApplicationInfo: *const VkApplicationInfo<'a>,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const c_char,
}

impl<'a> VkInstanceCreateInfo<'a> {
    pub fn new<T>(
        flags: T,
        application_info: &VkApplicationInfo<'a>,
        enabled_layer_names: &VkNames,
        enabled_extension_names: &VkNames,
    ) -> Self
    where
        T: Into<VkInstanceCreateFlagBits>,
    {
        VkInstanceCreateInfo {
            lt: PhantomData,
            sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            pApplicationInfo: application_info as *const _,
            enabledLayerCount: enabled_layer_names.c_names().len() as u32,
            ppEnabledLayerNames: enabled_layer_names.c_names().as_ptr(),
            enabledExtensionCount: enabled_extension_names.c_names().len() as u32,
            ppEnabledExtensionNames: enabled_extension_names.c_names().as_ptr(),
        }
    }
}

impl<'a> fmt::Debug for VkInstanceCreateInfo<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut enabled_layers_string = String::from("{{");
        let layers_slice: &[*const c_char] = unsafe {
            slice::from_raw_parts(self.ppEnabledLayerNames, self.enabledLayerCount as usize)
        };

        for layer in layers_slice {
            let cstr_layer = unsafe { CStr::from_ptr(*layer) };

            if let Ok(layer) = cstr_layer.to_str() {
                enabled_layers_string = format!("{} {},", enabled_layers_string, layer);
            }
        }

        enabled_layers_string = format!("{} }}", enabled_layers_string);

        let enabled_extensions_string = String::new();

        write!(
            f,
            "{{ sType: {:?}, pNext: {:?}, flags: {:?}, pApplicationInfo: {:?}, enabledLayers: {}, enabledExtensions: {} }}",
            self.sType,
            self.pNext,
            self.flags,
            self.pApplicationInfo,
            enabled_layers_string,
            enabled_extensions_string
        )
    }
}
