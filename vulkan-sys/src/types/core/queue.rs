
use crate::prelude::*;
use crate::SetupUSizeConv;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkQueue(usize);
SetupUSizeConv!(VkQueue);

/*
impl VkQueue {
    pub fn submit(&self, submits: &[VkSubmitInfo], fence: VkFence) -> Result<(), VkResult> {
        unsafe {
            let result = vkQueueSubmit(*self, submits.len() as u32, submits.as_ptr(), fence);

            if result == VK_SUCCESS {
                Ok(())
            } else {
                Err(result)
            }
        }
    }

    pub fn wait_idle(&self) -> Result<(), VkResult> {
        unsafe {
            let result = vkQueueWaitIdle(*self);

            if result == VK_SUCCESS {
                Ok(())
            } else {
                Err(result)
            }
        }
    }

    pub fn present_khr(&self, present_info: &VkPresentInfoKHR) -> Result<(), VkResult> {
        unsafe {
            let result = vkQueuePresentKHR(*self, present_info);

            if result == VK_SUCCESS {
                Ok(())
            } else {
                Err(result)
            }
        }
    }
}
*/
