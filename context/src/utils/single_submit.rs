use crate::prelude::*;

use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct SingleSubmit<'a, F, T>
where
    F: FnOnce(&Arc<CommandBuffer>) -> VerboseResult<T>,
{
    command_buffer: &'a Arc<CommandBuffer>,
    queue: &'a Arc<Mutex<Queue>>,
    f: F,

    timeout: Option<Duration>,
}

impl<'a, F, T> SingleSubmit<'a, F, T>
where
    F: FnOnce(&Arc<CommandBuffer>) -> VerboseResult<T>,
{
    pub fn builder(
        command_buffer: &'a Arc<CommandBuffer>,
        queue: &'a Arc<Mutex<Queue>>,
        f: F,
    ) -> Self {
        SingleSubmit {
            command_buffer,
            queue,
            f,

            timeout: None,
        }
    }

    pub fn wait_for_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);

        self
    }

    pub fn submit(self) -> VerboseResult<T> {
        self.command_buffer.begin(VkCommandBufferBeginInfo::new(
            VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
        ))?;

        let result = (self.f)(&self.command_buffer)?;

        self.command_buffer.end()?;

        let submit = SubmitInfo::default().add_command_buffer(self.command_buffer);
        let queue_lock = self.queue.lock()?;

        match self.timeout {
            Some(timeout) => {
                let fence = Fence::builder().build(self.command_buffer.device().clone())?;

                queue_lock.submit(Some(&fence), &[submit])?;

                self.command_buffer
                    .device()
                    .wait_for_fences(&[&fence], true, timeout)?;
            }
            None => {
                queue_lock.submit(None, &[submit])?;

                queue_lock.wait_idle()?;
            }
        }

        Ok(result)
    }
}
