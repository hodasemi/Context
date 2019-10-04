use utilities::prelude::*;

use crate::impl_vk_handle;
use crate::prelude::*;

use std::sync::Arc;

pub struct FenceBuilder {
    signaled: bool,
}

impl FenceBuilder {
    pub fn set_signaled(mut self, signaled: bool) -> Self {
        self.signaled = signaled;

        self
    }

    pub fn build(self, device: Arc<Device>) -> VerboseResult<Arc<Fence>> {
        let flag: VkFenceCreateFlagBits = if self.signaled {
            VK_FENCE_CREATE_SIGNALED_BIT.into()
        } else {
            0u32.into()
        };

        let fence_ci = VkFenceCreateInfo::new(flag);

        let fence = device.create_fence(&fence_ci)?;

        Ok(Arc::new(Fence { device, fence }))
    }
}

#[derive(Debug)]
pub struct Fence {
    device: Arc<Device>,
    fence: VkFence,
}

impl Fence {
    pub fn new() -> FenceBuilder {
        FenceBuilder { signaled: false }
    }

    pub fn reset(&self) -> bool {
        self.device.reset_fences(&[self.fence]).is_ok()
    }
}

unsafe impl Send for Fence {}
unsafe impl Sync for Fence {}

impl_vk_handle!(Fence, VkFence, fence);

impl Drop for Fence {
    fn drop(&mut self) {
        self.device.destroy_fence(self.fence);
    }
}
