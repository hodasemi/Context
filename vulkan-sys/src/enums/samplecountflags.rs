pub use VkSampleCountFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkSampleCountFlags {
    VK_SAMPLE_COUNT_1_BIT = 0x0000_0001,
    VK_SAMPLE_COUNT_2_BIT = 0x0000_0002,
    VK_SAMPLE_COUNT_4_BIT = 0x0000_0004,
    VK_SAMPLE_COUNT_8_BIT = 0x0000_0008,
    VK_SAMPLE_COUNT_16_BIT = 0x0000_0010,
    VK_SAMPLE_COUNT_32_BIT = 0x0000_0020,
    VK_SAMPLE_COUNT_64_BIT = 0x0000_0040,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct VkSampleCountFlagBits(u32);
SetupVkFlags!(VkSampleCountFlags, VkSampleCountFlagBits);

impl Default for VkSampleCountFlags {
    fn default() -> Self {
        VK_SAMPLE_COUNT_1_BIT
    }
}

impl Default for VkSampleCountFlagBits {
    fn default() -> Self {
        VK_SAMPLE_COUNT_1_BIT.into()
    }
}
