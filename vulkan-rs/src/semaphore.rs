use utilities::prelude::*;

use crate::impl_vk_handle;
use crate::prelude::*;

use std::sync::Arc;

#[derive(Debug)]
pub struct Semaphore {
    device: Arc<Device>,
    semaphore: VkSemaphore,
}

impl Semaphore {
    pub fn new(device: Arc<Device>) -> VerboseResult<Arc<Semaphore>> {
        let semaphore_ci = VkSemaphoreCreateInfo::new(VK_SEMAPHORE_CREATE_NULL_BIT);

        let semaphore = device.create_semaphore(&semaphore_ci)?;

        Ok(Arc::new(Semaphore { device, semaphore }))
    }
}

impl VulkanDevice for Semaphore {
    fn device(&self) -> &Arc<Device> {
        &self.device
    }
}

impl_vk_handle!(Semaphore, VkSemaphore, semaphore);

impl Drop for Semaphore {
    fn drop(&mut self) {
        self.device.destroy_semaphore(self.semaphore);
    }
}

use crate::{ffi::*, handle_ffi_result};

#[no_mangle]
pub extern "C" fn create_semaphore(device: *const Device) -> *const Semaphore {
    let device = unsafe { Arc::from_raw(device) };

    handle_ffi_result!(Semaphore::new(device))
}

#[no_mangle]
pub extern "C" fn destroy_semaphore(semaphore: *const Semaphore) {
    let _semaphore = unsafe { Arc::from_raw(semaphore) };
}
