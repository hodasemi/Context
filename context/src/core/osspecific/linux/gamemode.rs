#![allow(non_camel_case_types)]

use libloading::os::unix::Symbol as UnixSymbol;
use libloading::{Library, Symbol};

use crate::prelude::*;

use std::ffi::CStr;
use std::os::raw::c_char;

type gamemode_error_string = unsafe fn() -> *const c_char;
type gamemode_request_start = unsafe fn() -> i32;
type gamemode_request_end = unsafe fn() -> i32;

pub struct GameMode {
    _gamemode_lib: Library,

    disable_gamemode: UnixSymbol<gamemode_request_end>,
    get_error: UnixSymbol<gamemode_error_string>,
}

impl GameMode {
    pub fn new() -> VerboseResult<GameMode> {
        let lib = match Library::new("libgamemode.so") {
            Ok(lib) => lib,
            Err(_) => create_error!("failed loading gamemode.so".to_string()),
        };

        let enable_gamemode = {
            let symbol: Symbol<'_, gamemode_request_start> = unsafe {
                match lib.get(b"real_gamemode_request_start\0") {
                    Ok(func) => func,
                    Err(_) => create_error!(
                        "failed finding symbol real_gamemode_request_start".to_string()
                    ),
                }
            };

            unsafe { symbol.into_raw() }
        };

        let disable_gamemode = {
            let symbol: Symbol<'_, gamemode_request_end> = unsafe {
                match lib.get(b"real_gamemode_request_end\0") {
                    Ok(func) => func,
                    Err(_) => {
                        create_error!("failed finding symbol real_gamemode_request_end".to_string())
                    }
                }
            };

            unsafe { symbol.into_raw() }
        };

        let get_error = {
            let symbol: Symbol<'_, gamemode_error_string> = unsafe {
                match lib.get(b"real_gamemode_error_string\0") {
                    Ok(func) => func,
                    Err(_) => {
                        create_error!("failed finding symbol real_gamemode_error_string".to_string())
                    }
                }
            };

            unsafe { symbol.into_raw() }
        };

        if unsafe { (enable_gamemode)() } < 0 {
            create_error!(Self::string(unsafe { (get_error)() }));
        }

        println!("GameMode enabled");

        Ok(GameMode {
            _gamemode_lib: lib,

            disable_gamemode,
            get_error,
        })
    }

    fn string(msg: *const c_char) -> String {
        let c_str: &CStr = unsafe { CStr::from_ptr(msg) };
        let str_slice: &str = c_str.to_str().unwrap();
        str_slice.to_string()
    }
}

impl Drop for GameMode {
    fn drop(&mut self) {
        if unsafe { (self.disable_gamemode)() } < 0 {
            println!(
                "Error disabling GameMode: {}",
                Self::string(unsafe { (self.get_error)() })
            );

            return;
        }

        println!("GameMode disabled");
    }
}
