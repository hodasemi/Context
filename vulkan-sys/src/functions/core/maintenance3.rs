use crate::load_function_ptrs;
use crate::prelude::*;

use std::os::raw::c_void;

load_function_ptrs!(Maintenance3Functions, {
    vkGetDescriptorSetLayoutSupport(
        device: VkDevice,
        pCreateInfo: *const VkDescriptorSetLayoutCreateInfo,
        pSupport: *mut VkDescriptorSetLayoutSupport
    ) -> (),
});
