use std::ffi::CString;
use std::fmt;
use std::ops::Deref;
use std::os::raw::c_char;

#[derive(Clone, Eq, Hash)]
pub struct VkString {
    rust_text: String,
    cstring_text: CString,
}

impl VkString {
    pub fn new(text: &str) -> VkString {
        let owned = String::from(text);
        let cstring =
            CString::new(owned.clone()).expect(&format!("could not create CString ({})", text));

        VkString {
            rust_text: owned,
            cstring_text: cstring,
        }
    }

    pub fn as_ptr(&self) -> *const c_char {
        self.cstring_text.as_ptr()
    }

    pub fn as_str(&self) -> &str {
        &self.rust_text
    }

    pub fn as_string(&self) -> String {
        self.rust_text.clone()
    }

    pub fn into_string(self) -> String {
        self.rust_text
    }
}

impl Deref for VkString {
    type Target = String;

    fn deref(&self) -> &String {
        &self.rust_text
    }
}

impl PartialEq for VkString {
    fn eq(&self, other: &VkString) -> bool {
        self.rust_text == other.rust_text
    }
}

impl fmt::Debug for VkString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VkString {{ {} }}", self.rust_text)
    }
}
