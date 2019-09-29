use crate::prelude::*;

use std::ffi::CStr;
use std::fmt;
use std::marker::PhantomData;
use std::os::raw::{c_char, c_void};
use std::ptr;

#[repr(C)]
pub struct VkApplicationInfo<'a> {
    lt: PhantomData<&'a ()>,
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pApplicationName: *const c_char,
    pub applicationVersion: u32,
    pub pEngineName: *const c_char,
    pub engineVersion: u32,
    pub apiVersion: u32,
}

impl<'a> VkApplicationInfo<'a> {
    pub fn new(
        application_name: &'a VkString,
        application_version: u32,
        engine_name: &'a VkString,
        engine_version: u32,
        api_version: u32,
    ) -> Self {
        VkApplicationInfo {
            lt: PhantomData,
            sType: VK_STRUCTURE_TYPE_APPLICATION_INFO,
            pNext: ptr::null(),
            pApplicationName: application_name.as_ptr(),
            applicationVersion: application_version,
            pEngineName: engine_name.as_ptr(),
            engineVersion: engine_version,
            apiVersion: api_version,
        }
    }
}

impl<'a> fmt::Debug for VkApplicationInfo<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let application_name_c = unsafe { CStr::from_ptr(self.pApplicationName) };
        let application_name = match application_name_c.to_str() {
            Ok(name) => name,
            Err(_) => "",
        };

        let engine_name_c = unsafe { CStr::from_ptr(self.pEngineName) };
        let engine_name = match engine_name_c.to_str() {
            Ok(name) => name,
            Err(_) => "",
        };

        write!(
            f,
            "{{ sType: {:?}, pNext: {:?}, pApplicationName: {}, applicationVersion: {}, pEngineName: {}, engineVersion: {}, apiVersion: {} }}",
            self.sType,
            self.pNext,
            application_name,
            self.applicationVersion,
            engine_name,
            self.engineVersion,
            self.apiVersion
        )
    }
}
