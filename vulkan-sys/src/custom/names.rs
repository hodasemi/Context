use crate::prelude::*;

use std::os::raw::c_char;
use std::slice::Iter;

pub struct VkNames {
    r_names: Vec<VkString>,
    c_names: Vec<*const c_char>,
}

impl VkNames {
    pub fn new(names: &[VkString]) -> Self {
        let local: Vec<VkString> = names.iter().map(|s| s.clone()).collect();

        VkNames {
            c_names: local.iter().map(|s| s.as_ptr()).collect(),
            r_names: local,
        }
    }

    pub fn len(&self) -> usize {
        self.r_names.len()
    }

    pub fn iter(&self) -> Iter<'_, VkString> {
        self.r_names.iter()
    }

    pub fn c_names(&self) -> &Vec<*const c_char> {
        &self.c_names
    }
}
