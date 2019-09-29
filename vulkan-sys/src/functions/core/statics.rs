use crate::load_function_ptrs;
use crate::prelude::*;

use std::os::raw::c_char;
use std::os::raw::c_void;

load_function_ptrs!(StaticFunctions, {
    vkGetInstanceProcAddr(Instance: VkInstance, pName: *const c_char) -> PFN_vkVoidFunction,
});
