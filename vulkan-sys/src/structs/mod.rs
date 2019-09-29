pub mod amd;
pub mod core;
pub mod ext;
pub mod khr;
pub mod nv;

pub mod prelude;

pub trait PNext<T> {
    fn chain(&mut self, p_next: &T);
}

#[macro_export]
macro_rules! impl_pnext {
    ($implementor: ty, $struct_name: ident) => {
        impl PNext<$struct_name> for $implementor {
            fn chain(&mut self, p_next: &$struct_name) {
                self.pNext = p_next as *const $struct_name as *const c_void;
            }
        }
    };
    ($implementor: ty, $struct_name: ident, $block: block) => {
        impl PNext<$struct_name> for $implementor {
            fn chain(&mut self, p_next: &$struct_name) {
                $block
                self.pNext = p_next as *const $struct_name as *const c_void;
            }
        }
    };
}

use crate::prelude::*;

use std::ffi::CStr;
use std::os::raw::c_char;
use std::slice;

fn c_char_to_vkstring(text: *const c_char) -> Result<VkString, String> {
    let cstr = unsafe { CStr::from_ptr(text) };
    let str_ = match cstr.to_str() {
        Ok(str_) => str_,
        Err(_) => return Err("failed converting *const c_char".to_string()),
    };
    Ok(VkString::new(str_))
}

fn raw_to_slice<'a, T: Clone>(pointer: *const T, size: u32) -> &'a [T] {
    unsafe { slice::from_raw_parts(pointer, size as usize) }
}
