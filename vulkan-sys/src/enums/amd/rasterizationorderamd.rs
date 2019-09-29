pub use VkRasterizationOrderAMD::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkRasterizationOrderAMD {
    VK_RASTERIZATION_ORDER_STRICT_AMD = 0,
    VK_RASTERIZATION_ORDER_RELAXED_AMD = 1,
    VK_RASTERIZATION_ORDER_MAX_ENUM_AMD = 0x7FFF_FFFF,
}
