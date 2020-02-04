use crate::prelude::*;

use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct SingleSubmit;

impl SingleSubmit {
    pub fn builder<F>(
        command_buffer: &Arc<CommandBuffer>,
        queue: &Arc<Mutex<Queue>>,
        f: F,
        timeout: Duration,
    ) -> VerboseResult<()>
    where
        F: FnOnce(&Arc<CommandBuffer>) -> VerboseResult<()>,
    {
        command_buffer.begin(VkCommandBufferBeginInfo::new(
            VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
        ))?;

        f(command_buffer)?;

        command_buffer.end()?;

        let submit = SubmitInfo::default().add_command_buffer(command_buffer);
        let fence = Fence::builder().build(command_buffer.device().clone())?;

        let queue_lock = queue.lock()?;

        queue_lock.submit(Some(&fence), &[submit])?;

        command_buffer
            .device()
            .wait_for_fences(&[&fence], true, timeout)?;

        Ok(())
    }
}
