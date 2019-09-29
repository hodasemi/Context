pub use VkSharingMode::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VkSharingMode {
    VK_SHARING_MODE_EXCLUSIVE = 0,
    VK_SHARING_MODE_CONCURRENT = 1,
}

impl Default for VkSharingMode {
    fn default() -> Self {
        VK_SHARING_MODE_EXCLUSIVE
    }
}
