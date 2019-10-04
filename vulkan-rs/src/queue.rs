use utilities::prelude::*;

use crate::impl_vk_handle;
use crate::prelude::*;

use std::sync::Arc;

pub struct QueueRequestInfo {
    pub queue_create_info: VkDeviceQueueCreateInfo,
    pub queue_family_index: u32,
    pub queue_index: u32,
}

#[derive(Debug)]
pub struct Queue {
    device: Arc<Device>,
    queue: VkQueue,
    family_index: u32,
    queue_index: u32,
}

impl Queue {
    pub fn create_presentable_request_info(
        physical_device: &Arc<PhysicalDevice>,
        surface: &Arc<Surface>,
        queue_type: impl Into<VkQueueFlagBits>,
    ) -> VerboseResult<QueueRequestInfo> {
        let index =
            Self::find_presentable_queue_index(physical_device, surface, queue_type.into())?;

        let priorities = &[0.0f32];

        Ok(QueueRequestInfo {
            queue_create_info: VkDeviceQueueCreateInfo::new(0, index, priorities),
            queue_family_index: index,
            queue_index: 0,
        })
    }

    pub fn create_non_presentable_request_info(
        physical_device: &Arc<PhysicalDevice>,
        queue_type: impl Into<VkQueueFlagBits>,
    ) -> VerboseResult<QueueRequestInfo> {
        let index = Self::find_non_presentable_queue_index(physical_device, queue_type.into())?;

        let priorities = &[0.0f32];

        Ok(QueueRequestInfo {
            queue_create_info: VkDeviceQueueCreateInfo::new(0, index, priorities),
            queue_family_index: index,
            queue_index: 0,
        })
    }

    pub fn new(
        device: Arc<Device>,
        queue: VkQueue,
        family_index: u32,
        queue_index: u32,
    ) -> Arc<Queue> {
        Arc::new(Queue {
            device,
            queue,
            family_index,
            queue_index,
        })
    }

    pub fn family_index(&self) -> u32 {
        self.family_index
    }

    pub fn queue_index(&self) -> u32 {
        self.queue_index
    }

    pub fn submit(&self, fence: Option<&Arc<Fence>>, submits: &[SubmitInfo]) -> VerboseResult<()> {
        let submit_infos: Vec<VkSubmitInfo> = submits.iter().map(|s| s.as_vk_submit()).collect();

        let fence = match fence {
            Some(fence) => fence.vk_handle(),
            None => VkFence::NULL_HANDLE,
        };

        self.device
            .queue_submit(self.queue, submit_infos.as_slice(), fence)
    }

    pub fn present(
        &self,
        swapchains: &[&Arc<Swapchain>],
        image_indices: &[u32],
        wait_semaphores: &[&Arc<Semaphore>],
    ) -> VerboseResult<OutOfDate<()>> {
        let wait_semaphores: Vec<VkSemaphore> =
            wait_semaphores.iter().map(|sem| sem.vk_handle()).collect();

        let swapchains: Vec<VkSwapchainKHR> = swapchains
            .iter()
            .map(|swapchain| swapchain.vk_handle())
            .collect();

        let present_info = VkPresentInfoKHR::new(
            wait_semaphores.as_slice(),
            swapchains.as_slice(),
            image_indices,
            &mut [],
        );

        self.device.queue_present(self.queue, &present_info)
    }

    pub fn wait_idle(&self) -> VerboseResult<()> {
        self.device.queue_wait_idle(self.queue)
    }
}

impl Queue {
    fn find_presentable_queue_index(
        physical_device: &Arc<PhysicalDevice>,
        surface: &Arc<Surface>,
        flags: VkQueueFlagBits,
    ) -> VerboseResult<u32> {
        let surface = surface.vk_handle();
        let vk_physical_device = physical_device.vk_handle();

        let queue_family_properties = physical_device
            .instance()
            .physical_device_queue_family_properties(vk_physical_device);

        for (i, queue) in queue_family_properties.iter().enumerate() {
            if (queue.queueFlagBits & flags) == flags {
                let presentable = physical_device.instance().physical_device_surface_support(
                    vk_physical_device,
                    i as u32,
                    surface,
                )?;

                if presentable {
                    return Ok(i as u32);
                }
            }
        }

        create_error!("can't find device queue")
    }

    fn find_non_presentable_queue_index(
        physical_device: &Arc<PhysicalDevice>,
        flags: VkQueueFlagBits,
    ) -> VerboseResult<u32> {
        let vk_physical_device = physical_device.vk_handle();

        let queue_family_properties = physical_device
            .instance()
            .physical_device_queue_family_properties(vk_physical_device);

        for (i, queue) in queue_family_properties.iter().enumerate() {
            if (queue.queueFlagBits & flags) == flags {
                return Ok(i as u32);
            }
        }

        create_error!("can't find device queue")
    }
}

impl_vk_handle!(Queue, VkQueue, queue);

pub struct SubmitInfo {
    wait_semaphores: Vec<VkSemaphore>,
    wait_stages: Vec<VkPipelineStageFlagBits>,
    command_buffers: Vec<VkCommandBuffer>,
    signal_semaphores: Vec<VkSemaphore>,
}

impl SubmitInfo {
    pub fn new() -> SubmitInfo {
        SubmitInfo {
            wait_semaphores: Vec::new(),
            wait_stages: Vec::new(),
            command_buffers: Vec::new(),
            signal_semaphores: Vec::new(),
        }
    }

    pub fn add_wait_semaphore(mut self, wait_semaphore: impl VkHandle<VkSemaphore>) -> Self {
        self.wait_semaphores.push(wait_semaphore.vk_handle());

        self
    }

    pub fn add_wait_stage(mut self, wait_stage: impl Into<VkPipelineStageFlagBits>) -> Self {
        self.wait_stages.push(wait_stage.into());

        self
    }

    pub fn add_command_buffer(mut self, command_buffer: impl VkHandle<VkCommandBuffer>) -> Self {
        self.command_buffers.push(command_buffer.vk_handle());

        self
    }

    pub fn add_signal_semaphore(mut self, signal_semaphore: impl VkHandle<VkSemaphore>) -> Self {
        self.signal_semaphores.push(signal_semaphore.vk_handle());

        self
    }

    fn as_vk_submit(&self) -> VkSubmitInfo {
        VkSubmitInfo::new(
            self.wait_semaphores.as_slice(),
            self.wait_stages.as_slice(),
            self.command_buffers.as_slice(),
            self.signal_semaphores.as_slice(),
        )
    }
}
