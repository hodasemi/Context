pub use VkBool32::*;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum VkBool32 {
    VK_FALSE = 0,
    VK_TRUE = 1,
}

impl From<bool> for VkBool32 {
    fn from(b: bool) -> VkBool32 {
        if b {
            VK_TRUE
        } else {
            VK_FALSE
        }
    }
}

impl Default for VkBool32 {
    fn default() -> Self {
        VK_FALSE
    }
}
