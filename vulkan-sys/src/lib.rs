#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![deny(rust_2018_idioms)]

pub mod prelude;

pub mod custom;
pub mod enums;
pub mod functions;
pub mod structs;
pub mod types;

pub fn VK_MAKE_VERSION(major: u32, minor: u32, patch: u32) -> u32 {
    (major as u32) << 22 | (minor as u32) << 12 | (patch as u32)
}
