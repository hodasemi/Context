use crate::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct VkSparseImageMemoryBindInfo {
    pub image: VkImage,
    pub bindCount: u32,
    pub pBinds: *const VkSparseImageMemoryBind,
}
