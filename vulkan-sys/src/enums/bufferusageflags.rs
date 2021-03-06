pub use VkBufferUsageFlags::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkBufferUsageFlags {
    VK_BUFFER_USAGE_TRANSFER_SRC_BIT = 0x0000_0001,
    VK_BUFFER_USAGE_TRANSFER_DST_BIT = 0x0000_0002,
    VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT = 0x0000_0004,
    VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT = 0x0000_0008,
    VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT = 0x0000_0010,
    VK_BUFFER_USAGE_STORAGE_BUFFER_BIT = 0x0000_0020,
    VK_BUFFER_USAGE_INDEX_BUFFER_BIT = 0x0000_0040,
    VK_BUFFER_USAGE_VERTEX_BUFFER_BIT = 0x0000_0080,
    VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT = 0x0000_0100,
    VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_BUFFER_BIT_EXT = 0x0000_0800,
    VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT = 0x0000_1000,
    VK_BUFFER_USAGE_CONDITIONAL_RENDERING_BIT_EXT = 0x0000_0200,
    VK_BUFFER_USAGE_RAY_TRACING_BIT_NV = 0x0000_0400,
    VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_EXT = 0x0002_0000,
}

use crate::SetupVkFlags;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Hash, Default)]
pub struct VkBufferUsageFlagBits(u32);
SetupVkFlags!(VkBufferUsageFlags, VkBufferUsageFlagBits);
