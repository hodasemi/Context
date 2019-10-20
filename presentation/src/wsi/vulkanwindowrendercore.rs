use crate::{prelude::*, renderbackend::RenderBackend};

use super::windowsystemintegration::WindowSystemIntegration;

use utilities::prelude::*;
use vulkan_rs::prelude::*;

use std::cell::Cell;
use std::sync::{Arc, Mutex};
use std::u64;

pub struct VulkanWindowRenderCore {
    // driver provided images
    swapchain: Arc<Swapchain>,
    surface: Arc<Surface>,

    format: VkFormat,

    image_available_sem: Arc<Semaphore>,
    render_finished_sem: Arc<Semaphore>,
    render_fence: Arc<Fence>,

    render_backend: RenderBackend,

    current_image_index: Cell<usize>,
}

impl VulkanWindowRenderCore {
    pub fn new(
        wsi: &WindowSystemIntegration,
        device: &Arc<Device>,
        queue: &Arc<Mutex<Queue>>,
        vsync: bool,
    ) -> VerboseResult<(VulkanWindowRenderCore, TargetMode<()>)> {
        // check swapchain extension
        if !device.enabled_extensions().swapchain {
            create_error!("required swapchain extension not enabled!");
        }

        let surface = wsi.surface()?;

        if (surface.capabilities(device)?.supportedUsageFlagBits & VK_IMAGE_USAGE_TRANSFER_DST_BIT)
            == 0
        {
            create_error!(
                "requires the surface to be transfer destination, which isn't the case here"
            );
        }

        // create swapchain
        let swapchain = Swapchain::new(
            device.clone(),
            &surface,
            vsync,
            0,
            VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | VK_IMAGE_USAGE_TRANSFER_DST_BIT,
            1,
        )?;

        let mut swapchain_images = Vec::new();
        let (format, _) = surface.format_colorspace(&device)?;

        for image in swapchain.vk_images()? {
            swapchain_images.push(
                Image::from_preinitialized(
                    image,
                    format,
                    swapchain.width(),
                    swapchain.height(),
                    VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
                )
                .nearest_sampler()
                .build(device, queue)?,
            );
        }

        let render_sem = Semaphore::new(device.clone())?;
        let image_sem = Semaphore::new(device.clone())?;
        let fence = Fence::new().build(device.clone())?;

        let render_backend =
            RenderBackend::new(device, queue, TargetMode::Single(swapchain_images))?;

        let window_render_core = VulkanWindowRenderCore {
            swapchain,
            surface,

            format,

            render_finished_sem: render_sem,
            image_available_sem: image_sem,
            render_fence: fence,

            render_backend,

            current_image_index: Cell::new(0),
        };

        Ok((window_render_core, TargetMode::Single(())))
    }

    fn aquire_next_image_index(&self) -> VerboseResult<()> {
        loop {
            match self.swapchain.acquire_next_image(
                u64::MAX,
                Some(&self.image_available_sem),
                None,
            )? {
                OutOfDate::Ok(index) => {
                    self.current_image_index.set(index as usize);
                    break;
                }
                OutOfDate::OutOfDate => self.resize()?,
            }
        }

        Ok(())
    }

    fn resize(&self) -> VerboseResult<()> {
        self.swapchain.recreate()?;

        let (format, _) = self
            .surface
            .format_colorspace(&self.render_backend.device())?;

        let mut swapchain_images = Vec::new();

        for image in self.swapchain.vk_images()? {
            swapchain_images.push(
                Image::from_preinitialized(
                    image,
                    format,
                    self.swapchain.width(),
                    self.swapchain.height(),
                    VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
                )
                .nearest_sampler()
                .build(self.render_backend.device(), self.render_backend.queue())?,
            );
        }

        self.render_backend.resize(
            TargetMode::Single(swapchain_images),
            self.swapchain.width(),
            self.swapchain.height(),
        )?;

        Ok(())
    }
}

impl RenderCore for VulkanWindowRenderCore {
    fn format(&self) -> VkFormat {
        self.format
    }

    fn next_frame(&self) -> VerboseResult<bool> {
        self.aquire_next_image_index()?;

        let command_buffer = self
            .render_backend
            .render(TargetMode::Single(self.current_image_index.get()), None)?;

        let submits = &[SubmitInfo::new()
            .add_wait_semaphore(&self.image_available_sem)
            .add_wait_stage(VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT)
            .add_command_buffer(&command_buffer)
            .add_signal_semaphore(&self.render_finished_sem)];

        {
            let queue_lock = self.render_backend.queue().lock()?;

            queue_lock.submit(Some(&self.render_fence), submits)?;

            if let OutOfDate::OutOfDate = queue_lock.present(
                &[&self.swapchain],
                &[self.current_image_index.get() as u32],
                &[&self.render_finished_sem],
            )? {
                self.resize()?;
                self.render_fence.reset();
                return Ok(true);
            }
        }

        // make sure command_buffer is ready
        self.render_backend
            .device()
            .wait_for_fences(&[&self.render_fence], true, 2_000_000_000)?;
        self.render_fence.reset();

        Ok(true)
    }

    // scene handling
    fn add_scene(&self, scene: Arc<dyn TScene>) -> VerboseResult<()> {
        self.render_backend.add_scene(scene)
    }

    fn remove_scene(&self, scene: &Arc<dyn TScene>) -> VerboseResult<()> {
        self.render_backend.remove_scene(scene)
    }

    fn clear_scenes(&self) -> VerboseResult<()> {
        self.render_backend.clear_scenes()
    }

    // post process handling
    fn add_post_processing_routine(&self, post_process: Arc<dyn PostProcess>) -> VerboseResult<()> {
        self.render_backend
            .add_post_processing_routine(post_process)
    }

    fn remove_post_processing_routine(
        &self,
        post_process: &Arc<dyn PostProcess>,
    ) -> VerboseResult<()> {
        self.render_backend
            .remove_post_processing_routine(post_process)
    }

    fn clear_post_processing_routines(&self) -> VerboseResult<()> {
        self.render_backend.clear_post_processing_routines()
    }

    // getter
    fn image_count(&self) -> usize {
        self.render_backend.image_count()
    }

    fn images(&self) -> TargetMode<Vec<Arc<Image>>> {
        self.render_backend.images()
    }

    fn allocate_primary_buffer(&self) -> VerboseResult<Arc<CommandBuffer>> {
        self.render_backend.allocate_primary_buffer()
    }

    fn allocate_secondary_buffer(&self) -> VerboseResult<Arc<CommandBuffer>> {
        self.render_backend.allocate_secondary_buffer()
    }

    fn width(&self) -> u32 {
        self.swapchain.width()
    }

    fn height(&self) -> u32 {
        self.swapchain.height()
    }
}

impl std::fmt::Debug for VulkanWindowRenderCore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VulkanWindowRenderCore {{ }}")
    }
}
