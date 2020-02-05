use utilities::prelude::*;

use std::cell::RefCell;
use std::os::raw::{c_char, c_int};

#[macro_export]
macro_rules! handle_ffi_result {
    ($result: expr) => {
        match $result {
            Ok(value) => Arc::into_raw(value),
            Err(error) => {
                update_last_error(error);

                std::ptr::null()
            }
        }
    };
}

thread_local! {
    static LAST_ERROR:RefCell<Option<Box<String>>> = RefCell::new(None);
}

pub(crate) fn update_last_error(err: UtilError) {
    LAST_ERROR.with(|prev| {
        *prev.borrow_mut() = Some(Box::new(err.message()));
    });
}

pub(crate) fn take_last_error() -> Option<Box<String>> {
    LAST_ERROR.with(|prev| prev.borrow_mut().take())
}

#[no_mangle]
pub extern "C" fn last_error_length() -> c_int {
    LAST_ERROR.with(|prev| match *prev.borrow() {
        Some(ref err) => err.to_string().len() as c_int + 1,
        None => 0,
    })
}

#[no_mangle]
pub unsafe extern "C" fn last_error_message(buffer: *mut c_char, length: c_int) -> c_int {
    if buffer.is_null() {
        return -1;
    }

    let last_error = match take_last_error() {
        Some(err) => err,
        None => return 0,
    };

    let error_message = last_error.to_string();

    let buffer = std::slice::from_raw_parts_mut(buffer as *mut u8, length as usize);

    if error_message.len() >= buffer.len() {
        return -1;
    }

    std::ptr::copy_nonoverlapping(
        error_message.as_ptr(),
        buffer.as_mut_ptr(),
        error_message.len(),
    );

    // Add a trailing null so people using the string as a `char *` don't
    // accidentally read into garbage.
    buffer[error_message.len()] = 0;

    error_message.len() as c_int
}
