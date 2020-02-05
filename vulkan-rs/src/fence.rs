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
    pub fn builder() -> FenceBuilder {
        FenceBuilder { signaled: false }
    }

    pub fn reset(&self) -> bool {
        self.device.reset_fences(&[self.fence]).is_ok()
    }
}

impl VulkanDevice for Fence {
    fn device(&self) -> &Arc<Device> {
        &self.device
    }
}

impl_vk_handle!(Fence, VkFence, fence);

impl Drop for Fence {
    fn drop(&mut self) {
        self.device.destroy_fence(self.fence);
    }
}

use crate::{ffi::*, handle_ffi_result};

#[no_mangle]
pub extern "C" fn create_fence(signaled: bool, device: *const Device) -> *const Fence {
    let device = unsafe { Arc::from_raw(device) };

    let fence_res = Fence::builder().set_signaled(signaled).build(device);

    handle_ffi_result!(fence_res)
}

#[no_mangle]
pub extern "C" fn reset_fence(fence: *const Fence) -> bool {
    let fence = unsafe { Arc::from_raw(fence) };

    fence.reset()
}

#[no_mangle]
pub extern "C" fn destroy_fence(fence: *const Fence) {
    let _fence = unsafe { Arc::from_raw(fence) };
}
