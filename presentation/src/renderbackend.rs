use crate::prelude::*;

use cgmath::{Matrix4, SquareMatrix};
use utilities::prelude::*;
use vulkan_rs::prelude::*;

use std::ops::Deref;
use std::sync::{
    atomic::{AtomicUsize, Ordering::SeqCst},
    Arc, Mutex, RwLock,
};

pub enum TargetMode<T> {
    Single(T),
    Stereo(T, T),
}

impl<T> TargetMode<T> {
    pub fn single(&self) -> VerboseResult<&T> {
        match self {
            TargetMode::Single(s) => Ok(s),
            TargetMode::Stereo(_, _) => create_error!("single() on Stereo"),
        }
    }

    pub fn single_mut(&mut self) -> VerboseResult<&mut T> {
        match self {
            TargetMode::Single(s) => Ok(s),
            TargetMode::Stereo(_, _) => create_error!("single_mut() on Stereo"),
        }
    }

    pub fn stereo(&self) -> VerboseResult<(&T, &T)> {
        match self {
            TargetMode::Single(_) => create_error!("stereo() on Single"),
            TargetMode::Stereo(l, r) => Ok((l, r)),
        }
    }

    pub fn stereo_mut(&mut self) -> VerboseResult<(&mut T, &mut T)> {
        match self {
            TargetMode::Single(_) => create_error!("stereo_mut() on Single"),
            TargetMode::Stereo(l, r) => Ok((l, r)),
        }
    }
}

impl<T: Clone> Clone for TargetMode<T> {
    fn clone(&self) -> TargetMode<T> {
        match self {
            TargetMode::Single(t) => TargetMode::Single(t.clone()),
            TargetMode::Stereo(lhs, rhs) => TargetMode::Stereo(lhs.clone(), rhs.clone()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Eye {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub struct VRTransformations {
    pub proj: Matrix4<f32>,
    pub view: Matrix4<f32>,
}

impl VRTransformations {
    pub fn invert(self) -> VerboseResult<Self> {
        Ok(VRTransformations {
            proj: self
                .proj
                .invert()
                .ok_or("could not invert projection matrix")?,
            view: self.view.invert().ok_or("could not invert view matrix")?,
        })
    }
}

impl Default for VRTransformations {
    fn default() -> Self {
        VRTransformations {
            proj: Matrix4::identity(),
            view: Matrix4::identity(),
        }
    }
}

pub struct RenderBackend {
    device: Arc<Device>,
    queue: Arc<Mutex<Queue>>,

    // driver provided images
    swapchain_images: Mutex<TargetMode<Vec<Arc<Image>>>>,
    image_count: AtomicUsize,

    clear_color: RwLock<VkClearColorValue>,

    cmd_pool: Arc<CommandPool>,
    command_buffer: Arc<CommandBuffer>,

    scenes: Mutex<Vec<Arc<dyn TScene + Sync + Send>>>,
    post_processes: Mutex<Vec<Arc<dyn PostProcess + Sync + Send>>>,
}

impl RenderBackend {
    pub fn new(
        device: &Arc<Device>,
        queue: &Arc<Mutex<Queue>>,
        images: TargetMode<Vec<Arc<Image>>>,
    ) -> VerboseResult<RenderBackend> {
        let image_count = match &images {
            TargetMode::Single(images) => images.len(),
            TargetMode::Stereo(left_images, right_images) => {
                debug_assert!(left_images.len() == right_images.len());
                left_images.len()
            }
        };

        // command pool
        let command_pool = {
            let queue_lock = queue.lock()?;

            CommandPool::builder()
                .set_queue_family_index(queue_lock.family_index())
                .build(device.clone())?
        };

        // create a new command buffer
        let command_buffer = CommandPool::allocate_primary_buffer(&command_pool)?;

        Ok(RenderBackend {
            device: device.clone(),
            queue: queue.clone(),

            swapchain_images: Mutex::new(images),
            image_count: AtomicUsize::new(image_count),

            clear_color: RwLock::new(VkClearColorValue::float32([0.0, 0.0, 0.0, 1.0])),

            cmd_pool: command_pool,
            command_buffer,

            scenes: Mutex::new(Vec::new()),
            post_processes: Mutex::new(Vec::new()),
        })
    }

    pub(crate) fn required_image_usage() -> VkImageUsageFlagBits {
        VK_IMAGE_USAGE_TRANSFER_DST_BIT.into()
    }
}

impl RenderBackend {
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }

    pub fn queue(&self) -> &Arc<Mutex<Queue>> {
        &self.queue
    }

    pub fn set_clear_color(&self, clear_color: [f32; 4]) -> VerboseResult<()> {
        *self.clear_color.write()? = VkClearColorValue::float32(clear_color);

        Ok(())
    }

    pub fn render(&self, image_indices: TargetMode<usize>) -> VerboseResult<Arc<CommandBuffer>> {
        let scenes = self.scenes.lock()?;

        // update scenes
        for scene in scenes.iter() {
            scene.update()?;
        }

        // begin main command buffer
        self.command_buffer.begin(VkCommandBufferBeginInfo::new(
            VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
        ))?;

        // clear the current swapchain image
        {
            let swapchain_images = self.swapchain_images.lock()?;
            let target_layout = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR;
            let clear_color = self.clear_color.read()?.clone();

            match (&image_indices, swapchain_images.deref()) {
                (TargetMode::Single(image_index), TargetMode::Single(images)) => {
                    let swapchain_image = &images[*image_index];

                    Self::clear_image(
                        &self.command_buffer,
                        swapchain_image,
                        clear_color,
                        target_layout,
                    )?;
                }
                (
                    TargetMode::Stereo(left_image_index, right_image_index),
                    TargetMode::Stereo(left_images, right_images),
                ) => {
                    let left_image = &left_images[*left_image_index];
                    let right_image = &right_images[*right_image_index];

                    Self::clear_image(
                        &self.command_buffer,
                        left_image,
                        clear_color.clone(),
                        target_layout,
                    )?;

                    Self::clear_image(
                        &self.command_buffer,
                        right_image,
                        clear_color,
                        target_layout,
                    )?;
                }
                _ => create_error!("not fitting target modes!"),
            }
        }

        // make a call to the connected scenes
        for scene in scenes.iter() {
            scene.process(&self.command_buffer, &image_indices)?;
        }

        // post processing
        for post_process in self.post_processes.lock()?.iter() {
            post_process.process(&self.command_buffer, &image_indices)?;
        }

        self.command_buffer.end()?;

        Ok(self.command_buffer.clone())
    }

    pub fn resize(
        &self,
        images: TargetMode<Vec<Arc<Image>>>,
        width: u32,
        height: u32,
    ) -> VerboseResult<()> {
        self.image_count.store(
            match &images {
                TargetMode::Single(images) => images.len(),
                TargetMode::Stereo(left_images, right_images) => {
                    debug_assert!(left_images.len() == right_images.len());
                    left_images.len()
                }
            },
            SeqCst,
        );

        *self.swapchain_images.lock()? = images;

        for scene in self.scenes.lock()?.iter() {
            scene.resize()?;
        }

        for post_process in self.post_processes.lock()?.iter() {
            post_process.resize(width, height)?;
        }

        Ok(())
    }

    // scene handling
    pub fn add_scene(&self, scene: Arc<dyn TScene + Sync + Send>) -> VerboseResult<()> {
        let mut scenes = self.scenes.lock()?;

        // check if that scene is already present
        if scenes.iter().find(|s| Arc::ptr_eq(s, &scene)).is_none() {
            scenes.push(scene);
        }

        Ok(())
    }

    pub fn remove_scene(&self, scene: &Arc<dyn TScene + Sync + Send>) -> VerboseResult<()> {
        let mut scenes = self.scenes.lock()?;
        erase_arc(&mut scenes, scene);

        Ok(())
    }

    pub fn clear_scenes(&self) -> VerboseResult<()> {
        self.scenes.lock()?.clear();

        Ok(())
    }

    pub fn add_post_processing_routine(
        &self,
        post_process: Arc<dyn PostProcess + Sync + Send>,
    ) -> VerboseResult<()> {
        let mut post_processes = self.post_processes.lock()?;

        // only add if it isn't present already
        if post_processes
            .iter()
            .find(|p| Arc::ptr_eq(p, &post_process))
            .is_none()
        {
            post_processes.push(post_process);

            post_processes.sort_by(|lhs, rhs| lhs.priority().cmp(&rhs.priority()));
        }

        Ok(())
    }

    pub fn remove_post_processing_routine(
        &self,
        post_process: &Arc<dyn PostProcess + Sync + Send>,
    ) -> VerboseResult<()> {
        let mut post_processes = self.post_processes.lock()?;

        if let Some(index) = post_processes
            .iter()
            .position(|p| Arc::ptr_eq(p, post_process))
        {
            post_processes.remove(index);
        }

        Ok(())
    }

    pub fn clear_post_processing_routines(&self) -> VerboseResult<()> {
        self.post_processes.lock()?.clear();

        Ok(())
    }

    // getter
    pub fn image_count(&self) -> usize {
        self.image_count.load(SeqCst)
    }

    pub fn images(&self) -> VerboseResult<TargetMode<Vec<Arc<Image>>>> {
        Ok(self.swapchain_images.lock()?.clone())
    }

    pub fn allocate_primary_buffer(&self) -> VerboseResult<Arc<CommandBuffer>> {
        CommandPool::allocate_primary_buffer(&self.cmd_pool)
    }

    pub fn allocate_secondary_buffer(&self) -> VerboseResult<Arc<CommandBuffer>> {
        CommandPool::allocate_secondary_buffer(&self.cmd_pool)
    }
}

impl RenderBackend {
    #[inline]
    fn clear_image(
        command_buffer: &Arc<CommandBuffer>,
        image: &Arc<Image>,
        clear_color: VkClearColorValue,
        target_layout: VkImageLayout,
    ) -> VerboseResult<()> {
        command_buffer.set_full_image_layout(image, VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL)?;
        command_buffer.clear_color_image(image, clear_color)?;
        command_buffer.set_full_image_layout(image, target_layout)?;

        Ok(())
    }
}
