use crate::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct VkSparseBufferMemoryBindInfo {
    pub buffer: VkBuffer,
    pub bindCount: u32,
    pub pBinds: *const VkSparseMemoryBind,
}
