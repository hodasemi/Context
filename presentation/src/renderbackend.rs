use crate::prelude::*;

use cgmath::Matrix4;
use utilities::prelude::*;
use vulkan_rs::prelude::*;

use std::cell::{Cell, RefCell};
use std::ops::Deref;
use std::rc::Rc;
use std::slice;
use std::sync::Arc;

#[derive(Clone)]
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

pub struct RenderBackend {
    device: Arc<Device>,
    queue: Arc<Queue>,

    gui_render_pass: Arc<RenderPass>,

    // driver provided images
    swapchain_images: RefCell<TargetMode<Vec<Arc<Image>>>>,
    gui_framebuffers: RefCell<TargetMode<Vec<Arc<Framebuffer>>>>,
    image_count: Cell<usize>,

    cmd_pool: Arc<CommandPool>,
    command_buffer: Arc<CommandBuffer>,

    scenes: RefCell<Vec<Rc<dyn TScene>>>,

    resize_callback: RefCell<Option<Box<dyn Fn(u32, u32) -> VerboseResult<()>>>>,
    render_gui: RefCell<
        Option<
            Box<
                dyn Fn(
                    Option<Eye>,
                    usize,
                    &Arc<Framebuffer>,
                    &Arc<RenderPass>,
                ) -> VerboseResult<Arc<CommandBuffer>>,
            >,
        >,
    >,
}

impl RenderBackend {
    pub fn new(
        device: &Arc<Device>,
        queue: &Arc<Queue>,
        images: TargetMode<Vec<Arc<Image>>>,
        format: VkFormat,
    ) -> VerboseResult<RenderBackend> {
        let gui_render_pass = Self::create_gui_render_pass(device, format)?;

        let gui_framebuffers = Self::create_framebuffers(device, &images, &gui_render_pass)?;

        let image_count = match &images {
            TargetMode::Single(images) => images.len(),
            TargetMode::Stereo(left_images, right_images) => {
                debug_assert!(left_images.len() == right_images.len());
                left_images.len()
            }
        };

        // command pool
        let command_pool = CommandPool::new()
            .set_queue_family_index(queue.family_index())
            .build(device.clone())?;

        // create a new command buffer
        let command_buffer = CommandPool::allocate_primary_buffer(&command_pool)?;

        Ok(RenderBackend {
            device: device.clone(),
            queue: queue.clone(),

            gui_render_pass,

            swapchain_images: RefCell::new(images),
            gui_framebuffers: RefCell::new(gui_framebuffers),
            image_count: Cell::new(image_count),

            cmd_pool: command_pool,
            command_buffer,

            scenes: RefCell::new(Vec::new()),

            resize_callback: RefCell::new(None),
            render_gui: RefCell::new(None),
        })
    }
}

impl RenderBackend {
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }

    pub fn queue(&self) -> &Arc<Queue> {
        &self.queue
    }

    pub fn render(
        &self,
        image_indices: TargetMode<usize>,
        vr_data: Option<TargetMode<VRTransformations>>,
    ) -> VerboseResult<Arc<CommandBuffer>> {
        let scenes = self.scenes.try_borrow()?;

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
            let swapchain_images = self.swapchain_images.borrow();

            match (&image_indices, swapchain_images.deref()) {
                (TargetMode::Single(image_index), TargetMode::Single(images)) => {
                    let swapchain_image = &images[*image_index];

                    Self::clear_image(
                        &self.command_buffer,
                        swapchain_image,
                        VkClearColorValue::float32([0.0, 0.0, 0.0, 1.0]),
                    );
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
                        VkClearColorValue::float32([1.0, 0.0, 0.0, 1.0]),
                    );

                    Self::clear_image(
                        &self.command_buffer,
                        right_image,
                        VkClearColorValue::float32([0.0, 1.0, 0.0, 1.0]),
                    );
                }
                _ => create_error!("not fitting target modes!"),
            }
        }

        // make a call to the connected scenes
        for scene in scenes.iter() {
            scene.process(&self.command_buffer, &image_indices, &vr_data)?;
        }

        // gui rendering
        if let Some(render_gui) = self.render_gui.try_borrow()?.as_ref() {
            let gui_framebuffers = self.gui_framebuffers.borrow();

            match (&image_indices, gui_framebuffers.deref()) {
                (TargetMode::Single(image_index), TargetMode::Single(framebuffers)) => {
                    let framebuffer = &framebuffers[*image_index];

                    self.command_buffer.begin_render_pass_full(
                        &self.gui_render_pass,
                        framebuffer,
                        &[],
                        VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS,
                    );

                    self.command_buffer.execute_commands(&[&render_gui(
                        None,
                        *image_index,
                        framebuffer,
                        &self.gui_render_pass,
                    )?]);

                    self.command_buffer.end_render_pass();
                }
                (
                    TargetMode::Stereo(left_image_index, right_image_index),
                    TargetMode::Stereo(left_framebuffers, right_framebuffers),
                ) => {
                    // submit left framebuffer
                    let left_framebuffer = &left_framebuffers[*left_image_index];

                    self.command_buffer.begin_render_pass_full(
                        &self.gui_render_pass,
                        left_framebuffer,
                        &[],
                        VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS,
                    );

                    self.command_buffer.execute_commands(&[&render_gui(
                        Some(Eye::Left),
                        *left_image_index,
                        left_framebuffer,
                        &self.gui_render_pass,
                    )?]);

                    self.command_buffer.end_render_pass();

                    // submit right framebuffer
                    let right_framebuffer = &right_framebuffers[*right_image_index];

                    self.command_buffer.begin_render_pass_full(
                        &self.gui_render_pass,
                        right_framebuffer,
                        &[],
                        VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS,
                    );

                    self.command_buffer.execute_commands(&[&render_gui(
                        Some(Eye::Right),
                        *right_image_index,
                        right_framebuffer,
                        &self.gui_render_pass,
                    )?]);

                    self.command_buffer.end_render_pass();
                }
                _ => create_error!("not fitting target modes!"),
            }
        }

        self.command_buffer.end()?;

        Ok(self.command_buffer.clone())
    }

    pub fn resize(
        &self,
        images: TargetMode<Vec<Arc<Image>>>,
        format: VkFormat,
        width: u32,
        height: u32,
    ) -> VerboseResult<()> {
        self.image_count.set(match &images {
            TargetMode::Single(images) => images.len(),
            TargetMode::Stereo(left_images, right_images) => {
                debug_assert!(left_images.len() == right_images.len());
                left_images.len()
            }
        });

        let gui_render_pass = Self::create_gui_render_pass(&self.device, format)?;

        let gui_framebuffers = Self::create_framebuffers(&self.device, &images, &gui_render_pass)?;

        *self.gui_framebuffers.try_borrow_mut()? = gui_framebuffers;
        *self.swapchain_images.try_borrow_mut()? = images;

        if let Some(resize_callback) = self.resize_callback.try_borrow()?.as_ref() {
            resize_callback(width, height)?;
        }

        for scene in self.scenes.try_borrow()?.iter() {
            scene.resize()?;
        }

        Ok(())
    }

    // scene handling
    pub fn add_scene(&self, scene: Rc<dyn TScene>) -> VerboseResult<()> {
        let mut scenes = self.scenes.try_borrow_mut()?;

        // check if that scene is already present
        if scenes.iter().find(|s| Rc::ptr_eq(s, &scene)).is_none() {
            scenes.push(scene);
        }

        Ok(())
    }

    pub fn remove_scene(&self, scene: &Rc<dyn TScene>) -> VerboseResult<()> {
        let mut scenes = self.scenes.try_borrow_mut()?;
        erase_rc(&mut scenes, scene);

        Ok(())
    }

    pub fn clear_scenes(&self) -> VerboseResult<()> {
        self.scenes.try_borrow_mut()?.clear();

        Ok(())
    }

    // callbacks
    pub fn set_resize_callback(
        &self,
        resize_callback: Option<Box<dyn Fn(u32, u32) -> VerboseResult<()>>>,
    ) -> VerboseResult<()> {
        *self.resize_callback.try_borrow_mut()? = resize_callback;

        Ok(())
    }

    pub fn set_gui_callback(
        &self,
        render_gui: Option<
            Box<
                dyn Fn(
                    Option<Eye>,
                    usize,
                    &Arc<Framebuffer>,
                    &Arc<RenderPass>,
                ) -> VerboseResult<Arc<CommandBuffer>>,
            >,
        >,
    ) -> VerboseResult<()> {
        *self.render_gui.try_borrow_mut()? = render_gui;

        Ok(())
    }

    // getter
    pub fn image_count(&self) -> usize {
        self.image_count.get()
    }

    pub fn images(&self) -> TargetMode<Vec<Arc<Image>>> {
        self.swapchain_images.borrow().clone()
    }

    pub fn gui_render_pass(&self) -> &Arc<RenderPass> {
        &self.gui_render_pass
    }

    // pub fn current_gui_framebuffer(&self, image_index: usize) -> Arc<Framebuffer> {
    //     self.gui_framebuffers.borrow()[image_index].clone()
    // }

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
    ) {
        command_buffer.set_full_image_layout(image, VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL);
        command_buffer.clear_color_image(image, clear_color);
        command_buffer.set_full_image_layout(image, VK_IMAGE_LAYOUT_PRESENT_SRC_KHR);
    }

    /// Creates a simple render pass for gui rendering
    /// Only color framebuffer is attached
    fn create_gui_render_pass(
        device: &Arc<Device>,
        final_format: VkFormat,
    ) -> VerboseResult<Arc<RenderPass>> {
        let target_reference = VkAttachmentReference {
            attachment: 0,
            layout: VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
        };

        let subpass_descriptions = [VkSubpassDescription::new(
            0,
            &[],
            slice::from_ref(&target_reference),
            &[],
            None,
            &[],
        )];

        let attachments = [VkAttachmentDescription::new(
            0,
            final_format,
            VK_SAMPLE_COUNT_1_BIT,
            VK_ATTACHMENT_LOAD_OP_LOAD,
            VK_ATTACHMENT_STORE_OP_STORE,
            VK_ATTACHMENT_LOAD_OP_DONT_CARE,
            VK_ATTACHMENT_STORE_OP_DONT_CARE,
            VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
            VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
        )];

        let dependencies = [
            VkSubpassDependency::new(
                VK_SUBPASS_EXTERNAL,
                0,
                VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT,
                VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
                VK_ACCESS_MEMORY_READ_BIT,
                VK_ACCESS_COLOR_ATTACHMENT_READ_BIT | VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
                VK_DEPENDENCY_BY_REGION_BIT,
            ),
            VkSubpassDependency::new(
                0,
                VK_SUBPASS_EXTERNAL,
                VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
                VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT,
                VK_ACCESS_COLOR_ATTACHMENT_READ_BIT | VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
                VK_ACCESS_MEMORY_READ_BIT,
                VK_DEPENDENCY_BY_REGION_BIT,
            ),
        ];

        let renderpass = RenderPass::new(
            device.clone(),
            &subpass_descriptions,
            &attachments,
            &dependencies,
        )?;

        Ok(renderpass)
    }

    fn create_framebuffers(
        device: &Arc<Device>,
        target_images: &TargetMode<Vec<Arc<Image>>>,
        render_pass: &Arc<RenderPass>,
    ) -> VerboseResult<TargetMode<Vec<Arc<Framebuffer>>>> {
        // closure to create array of framebuffer from array of images
        let create_framebuffer = |device: &Arc<Device>,
                                  images: &Vec<Arc<Image>>,
                                  render_pass: &Arc<RenderPass>|
         -> VerboseResult<Vec<Arc<Framebuffer>>> {
            let mut framebuffers = Vec::with_capacity(images.len());

            for image in images.iter() {
                image.convert_layout(VK_IMAGE_LAYOUT_PRESENT_SRC_KHR)?;

                framebuffers.push(
                    Framebuffer::new()
                        .set_render_pass(render_pass)
                        .add_attachment(&image)
                        .build(device.clone())?,
                )
            }

            Ok(framebuffers)
        };

        match target_images {
            TargetMode::Single(images) => {
                let framebuffers = create_framebuffer(device, images, render_pass)?;

                Ok(TargetMode::Single(framebuffers))
            }
            TargetMode::Stereo(left_images, right_images) => {
                let left_framebuffers = create_framebuffer(device, left_images, render_pass)?;
                let right_framebuffers = create_framebuffer(device, right_images, render_pass)?;

                Ok(TargetMode::Stereo(left_framebuffers, right_framebuffers))
            }
        }
    }
}
