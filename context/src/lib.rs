#![deny(rust_2018_idioms)]

pub mod core;
pub mod prelude;
pub mod utils;

#[cfg(feature = "user_interface")]
pub mod gui;
