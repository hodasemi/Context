use crate::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct VkSubpassDependency {
    pub srcSubpass: u32,
    pub dstSubpass: u32,
    pub srcStageMask: VkPipelineStageFlagBits,
    pub dstStageMask: VkPipelineStageFlagBits,
    pub srcAccessMask: VkAccessFlagBits,
    pub dstAccessMask: VkAccessFlagBits,
    pub dependencyFlagBits: VkDependencyFlagBits,
}

impl VkSubpassDependency {
    pub fn new<S, T, U, V, W>(
        src_subpass: u32,
        dst_subpass: u32,
        src_stage_mask: S,
        dst_stage_mask: T,
        src_access_mask: U,
        dst_access_mask: V,
        dependency_flags: W,
    ) -> Self
    where
        S: Into<VkPipelineStageFlagBits>,
        T: Into<VkPipelineStageFlagBits>,
        U: Into<VkAccessFlagBits>,
        V: Into<VkAccessFlagBits>,
        W: Into<VkDependencyFlagBits>,
    {
        VkSubpassDependency {
            srcSubpass: src_subpass,
            dstSubpass: dst_subpass,
            srcStageMask: src_stage_mask.into(),
            dstStageMask: dst_stage_mask.into(),
            srcAccessMask: src_access_mask.into(),
            dstAccessMask: dst_access_mask.into(),
            dependencyFlagBits: dependency_flags.into(),
        }
    }
}
