pub use VkCompositeAlphaFlagsKHR::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkCompositeAlphaFlagsKHR {
    VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR = 0x00000001,
    VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR = 0x00000002,
    VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR = 0x00000004,
    VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR = 0x00000008,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkCompositeAlphaFlagBitsKHR(u32);
SetupVkFlags!(VkCompositeAlphaFlagsKHR, VkCompositeAlphaFlagBitsKHR);

impl Default for VkCompositeAlphaFlagBitsKHR {
    fn default() -> Self {
        VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR.into()
    }
}
