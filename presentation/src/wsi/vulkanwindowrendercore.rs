use crate::{prelude::*, renderbackend::RenderBackend, RenderCoreCreateInfo};

use super::windowsystemintegration::WindowSystemIntegration;

use utilities::prelude::*;
use vulkan_rs::prelude::*;

use std::sync::{
    atomic::{AtomicUsize, Ordering::SeqCst},
    Arc, Mutex,
};
use std::time::Duration;
use std::u64;

pub struct VulkanWindowRenderCore {
    // driver provided images
    swapchain: Arc<Swapchain>,
    _surface: Arc<Surface>,

    format: VkFormat,
    usage: VkImageUsageFlagBits,

    image_available_sem: Arc<Semaphore>,
    render_finished_sem: Arc<Semaphore>,
    render_fence: Arc<Fence>,

    render_backend: RenderBackend,

    current_image_index: AtomicUsize,
}

impl VulkanWindowRenderCore {
    pub fn new(
        wsi: &WindowSystemIntegration,
        device: &Arc<Device>,
        queue: &Arc<Mutex<Queue>>,
        create_info: RenderCoreCreateInfo,
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

        let usage = create_info.usage | RenderBackend::required_image_usage();

        // create swapchain
        let swapchain = Swapchain::new(
            device.clone(),
            &surface,
            create_info.vsync,
            2,
            usage,
            create_info.format,
            1,
        )?;

        let swapchain_images = Self::create_swapchain_images(&swapchain, device, queue, usage)?;

        let render_sem = Semaphore::new(device.clone())?;
        let image_sem = Semaphore::new(device.clone())?;
        let fence = Fence::builder().build(device.clone())?;

        let render_backend =
            RenderBackend::new(device, queue, TargetMode::Single(swapchain_images))?;

        let window_render_core = VulkanWindowRenderCore {
            format: swapchain.format()?,
            usage: usage,

            swapchain,
            _surface: surface,

            render_finished_sem: render_sem,
            image_available_sem: image_sem,
            render_fence: fence,

            render_backend,

            current_image_index: AtomicUsize::new(0),
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
                    self.current_image_index.store(index as usize, SeqCst);
                    break;
                }
                OutOfDate::OutOfDate => self.resize()?,
            }
        }

        Ok(())
    }

    fn resize(&self) -> VerboseResult<()> {
        self.swapchain.recreate()?;

        let swapchain_images = Self::create_swapchain_images(
            &self.swapchain,
            self.render_backend.device(),
            self.render_backend.queue(),
            self.usage,
        )?;

        self.render_backend.resize(
            TargetMode::Single(swapchain_images),
            self.swapchain.width(),
            self.swapchain.height(),
        )?;

        Ok(())
    }

    fn create_swapchain_images(
        swapchain: &Arc<Swapchain>,
        device: &Arc<Device>,
        queue: &Arc<Mutex<Queue>>,
        usage: impl Into<VkImageUsageFlagBits>,
    ) -> VerboseResult<Vec<Arc<Image>>> {
        let usage = usage.into();
        let format = swapchain.format()?;
        let tiling = VK_IMAGE_TILING_OPTIMAL;

        if !Image::check_configuration(device, tiling, swapchain.format()?, usage) {
            create_error!(format!(
                "wrong config: {:?}, {:?}, {:?}",
                tiling, format, usage
            ));
        }

        let mut swapchain_images = Vec::new();

        for image in swapchain.vk_images()? {
            swapchain_images.push(
                Image::from_preinitialized(
                    image,
                    format,
                    swapchain.width(),
                    swapchain.height(),
                    VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
                    usage,
                )
                .nearest_sampler()
                .build(device, queue)?,
            );
        }

        Ok(swapchain_images)
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
            .render(TargetMode::Single(self.current_image_index.load(SeqCst)))?;

        let submits = &[SubmitInfo::default()
            .add_wait_semaphore(&self.image_available_sem)
            .add_wait_stage(VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT)
            .add_command_buffer(&command_buffer)
            .add_signal_semaphore(&self.render_finished_sem)];

        if let OutOfDate::OutOfDate = {
            let queue_lock = self.render_backend.queue().lock()?;

            queue_lock.submit(Some(&self.render_fence), submits)?;

            queue_lock.present(
                &[&self.swapchain],
                &[self.current_image_index.load(SeqCst) as u32],
                &[&self.render_finished_sem],
            )?
        } {
            self.resize()?;
            self.render_fence.reset();
            return Ok(true);
        }

        // make sure command_buffer is ready
        self.render_backend.device().wait_for_fences(
            &[&self.render_fence],
            true,
            Duration::from_secs(10),
        )?;
        self.render_fence.reset();

        Ok(true)
    }

    fn set_clear_color(&self, color: [f32; 4]) -> VerboseResult<()> {
        self.render_backend.set_clear_color(color)
    }

    // scene handling
    fn add_scene(&self, scene: Arc<dyn TScene + Sync + Send>) -> VerboseResult<()> {
        self.render_backend.add_scene(scene)
    }

    fn remove_scene(&self, scene: &Arc<dyn TScene + Sync + Send>) -> VerboseResult<()> {
        self.render_backend.remove_scene(scene)
    }

    fn clear_scenes(&self) -> VerboseResult<()> {
        self.render_backend.clear_scenes()
    }

    // post process handling
    fn add_post_processing_routine(
        &self,
        post_process: Arc<dyn PostProcess + Sync + Send>,
    ) -> VerboseResult<()> {
        self.render_backend
            .add_post_processing_routine(post_process)
    }

    fn remove_post_processing_routine(
        &self,
        post_process: &Arc<dyn PostProcess + Sync + Send>,
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

    fn images(&self) -> VerboseResult<TargetMode<Vec<Arc<Image>>>> {
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

    fn transformations(&self) -> VerboseResult<Option<(VRTransformations, VRTransformations)>> {
        Ok(None)
    }
}

impl std::fmt::Debug for VulkanWindowRenderCore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VulkanWindowRenderCore {{ }}")
    }
}
