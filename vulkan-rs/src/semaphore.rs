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

unsafe impl Sync for Semaphore {}
unsafe impl Send for Semaphore {}

impl_vk_handle!(Semaphore, VkSemaphore, semaphore);

impl Drop for Semaphore {
    fn drop(&mut self) {
        self.device.destroy_semaphore(self.semaphore);
    }
}
