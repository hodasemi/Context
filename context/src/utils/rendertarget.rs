use crate::prelude::*;

use std::sync::{Arc, Mutex};

pub enum ClearValue {
    Color([f32; 4]),
    Depth(f32, u32),
}

pub struct CustomTarget {
    pub usage: VkImageUsageFlagBits,
    pub format: VkFormat,
    pub clear_on_load: bool,
    pub store_on_save: bool,
    pub attach_sampler: bool,
    pub clear_value: ClearValue,
}

impl CustomTarget {
    pub fn depth() -> CustomTarget {
        CustomTarget {
            usage: VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT.into(),
            format: VK_FORMAT_D16_UNORM,
            clear_on_load: true,
            store_on_save: false,
            attach_sampler: false,
            clear_value: ClearValue::Depth(1.0, 0),
        }
    }
}

pub struct RenderTargetBuilder<'a> {
    width: u32,
    height: u32,
    sample_count: VkSampleCountFlags,

    target_infos: Vec<CustomTarget>,

    prepared_targets: Option<(&'a [Arc<Image>], usize, [f32; 4])>,
    resolve_targets: Option<&'a [Arc<Image>]>,
}

impl<'a> RenderTargetBuilder<'a> {
    pub fn set_sample_count(mut self, sample_count: VkSampleCountFlags) -> Self {
        self.sample_count = sample_count;

        self
    }

    pub fn add_target_info(mut self, target: CustomTarget) -> Self {
        self.target_infos.push(target);

        self
    }

    pub fn set_prepared_targets(
        mut self,
        prepared_targets: &'a [Arc<Image>],
        target_index: usize,
        clear_color: impl Into<[f32; 4]>,
    ) -> Self {
        self.prepared_targets = Some((prepared_targets, target_index, clear_color.into()));

        self
    }

    pub fn set_resolve_targets(mut self, resolve_targets: &'a [Arc<Image>]) -> Self {
        self.resolve_targets = Some(resolve_targets);

        self
    }

    pub fn build(
        self,
        device: &Arc<Device>,
        queue: &Arc<Mutex<Queue>>,
    ) -> VerboseResult<RenderTarget> {
        let (render_pass, images, clear_values) =
            self.create_images_and_renderpass(device, queue)?;

        let mut framebuffers = Vec::new();

        match (self.resolve_targets, self.prepared_targets) {
            (Some(resolve_targets), Some((prepared_targets, index, _))) => {
                debug_assert!(prepared_targets.len() == resolve_targets.len());

                for (i, resolve_target) in resolve_targets.iter().enumerate() {
                    let ref_images =
                        Self::insert_prepared_target(&images, &prepared_targets[i], index);

                    let mut framebuffer_builder = Framebuffer::builder()
                        .set_render_pass(&render_pass)
                        .set_width(self.width)
                        .set_height(self.height);

                    for ref_image in ref_images {
                        framebuffer_builder = framebuffer_builder.add_attachment(ref_image);
                    }

                    let framebuffer = framebuffer_builder
                        .add_attachment(resolve_target)
                        .build(device.clone())?;

                    framebuffers.push(framebuffer);
                }
            }
            (Some(resolve_targets), None) => {
                for resolve_target in resolve_targets {
                    let mut framebuffer_builder = Framebuffer::builder()
                        .set_render_pass(&render_pass)
                        .set_width(self.width)
                        .set_height(self.height);

                    for image in &images {
                        framebuffer_builder = framebuffer_builder.add_attachment(image);
                    }

                    let framebuffer = framebuffer_builder
                        .add_attachment(&resolve_target)
                        .build(device.clone())?;

                    framebuffers.push(framebuffer);
                }
            }
            (None, Some((prepared_targets, index, _))) => {
                for prepared_target in prepared_targets {
                    let ref_images = Self::insert_prepared_target(&images, &prepared_target, index);

                    let mut framebuffer_builder = Framebuffer::builder()
                        .set_render_pass(&render_pass)
                        .set_width(self.width)
                        .set_height(self.height);

                    for image in &ref_images {
                        framebuffer_builder = framebuffer_builder.add_attachment(image);
                    }

                    let framebuffer = framebuffer_builder.build(device.clone())?;

                    framebuffers.push(framebuffer);
                }
            }
            (None, None) => {
                let mut framebuffer_builder = Framebuffer::builder()
                    .set_render_pass(&render_pass)
                    .set_width(self.width)
                    .set_height(self.height);

                for image in &images {
                    framebuffer_builder = framebuffer_builder.add_attachment(image);
                }

                let framebuffer = framebuffer_builder.build(device.clone())?;

                framebuffers.push(framebuffer);
            }
        }

        Ok(RenderTarget {
            render_pass,

            framebuffers,
            images,

            extent: VkExtent2D {
                width: self.width,
                height: self.height,
            },
            clear_values,
        })
    }
}

pub struct RenderTarget {
    render_pass: Arc<RenderPass>,

    framebuffers: Vec<Arc<Framebuffer>>,
    images: Vec<Arc<Image>>,

    extent: VkExtent2D,
    clear_values: Vec<VkClearValue>,
}

impl RenderTarget {
    pub fn new<'a>(width: u32, height: u32) -> RenderTargetBuilder<'a> {
        RenderTargetBuilder {
            width,
            height,
            sample_count: VK_SAMPLE_COUNT_1_BIT,

            target_infos: Vec::new(),

            prepared_targets: None,
            resolve_targets: None,
        }
    }

    pub fn render_pass(&self) -> &Arc<RenderPass> {
        &self.render_pass
    }

    pub fn framebuffer(&self, index: usize) -> &Arc<Framebuffer> {
        &self.framebuffers[index]
    }

    pub fn images(&self) -> &Vec<Arc<Image>> {
        &self.images
    }

    pub fn begin(
        &self,
        command_buffer: &Arc<CommandBuffer>,
        subpass_content: VkSubpassContents,
        framebuffer_index: usize,
    ) {
        let renderpass_begin = VkRenderPassBeginInfo::new(
            self.render_pass.vk_handle(),
            self.framebuffers[framebuffer_index].vk_handle(),
            VkRect2D {
                offset: VkOffset2D { x: 0, y: 0 },
                extent: self.extent,
            },
            self.clear_values.as_slice(),
        );

        command_buffer.begin_render_pass(renderpass_begin, subpass_content);
    }

    pub fn end(&self, command_buffer: &Arc<CommandBuffer>) {
        command_buffer.end_render_pass();
    }
}

impl<'a> RenderTargetBuilder<'a> {
    fn insert_prepared_target<'b>(
        images: &'b [Arc<Image>],
        prepared_target: &'b Arc<Image>,
        index: usize,
    ) -> Vec<&'b Arc<Image>> {
        let mut ref_images = Vec::new();

        for (i, image) in images.iter().enumerate() {
            if i == index {
                ref_images.push(prepared_target);
            }

            ref_images.push(image);
        }

        ref_images
    }

    fn create_images_and_renderpass(
        &self,
        device: &Arc<Device>,
        queue: &Arc<Mutex<Queue>>,
    ) -> VerboseResult<(Arc<RenderPass>, Vec<Arc<Image>>, Vec<VkClearValue>)> {
        // check for correct sample count
        let checked_sample_count = device.max_supported_sample_count(self.sample_count);

        // throw an error if we don't use muultisampling and have an resolve target
        if checked_sample_count == VK_SAMPLE_COUNT_1_BIT && self.resolve_targets.is_some() {
            create_error!("Sample count 1 and using resolve target is not supported");
        }

        let mut images = Vec::new();
        let mut clear_values = Vec::new();

        // init values for renderpass
        let mut color_references = Vec::new();
        let mut resolve_reference = None;
        let mut depth_reference = None;
        let mut attachments = Vec::new();

        let mut attachment_count = 0;

        for (i, target_info) in self.target_infos.iter().enumerate() {
            // check for prepared images and their index
            if let Some((prepared_images, index, clear_color)) = self.prepared_targets {
                if i == index {
                    // assume prepared images are always color attachments
                    clear_values.push(VkClearValue::color(VkClearColorValue::float32(clear_color)));

                    // add color attachment
                    attachments.push(VkAttachmentDescription::new(
                        0,
                        prepared_images[0].vk_format(),
                        VK_SAMPLE_COUNT_1_BIT,
                        VK_ATTACHMENT_LOAD_OP_CLEAR,
                        VK_ATTACHMENT_STORE_OP_STORE,
                        VK_ATTACHMENT_LOAD_OP_DONT_CARE,
                        VK_ATTACHMENT_STORE_OP_DONT_CARE,
                        prepared_images[0].image_layout()?,
                        prepared_images[0].image_layout()?,
                    ));

                    // add color reference
                    color_references.push(VkAttachmentReference {
                        attachment: attachment_count,
                        layout: VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
                    });

                    attachment_count += 1;
                }
            }

            let clear_operation = if target_info.clear_on_load {
                VK_ATTACHMENT_LOAD_OP_CLEAR
            } else {
                VK_ATTACHMENT_LOAD_OP_LOAD
            };

            let store_operation = if target_info.store_on_save {
                VK_ATTACHMENT_STORE_OP_STORE
            } else {
                VK_ATTACHMENT_STORE_OP_DONT_CARE
            };

            // push clear values
            match target_info.clear_value {
                ClearValue::Color(color) => {
                    clear_values.push(VkClearValue::color(VkClearColorValue::float32(color)))
                }
                ClearValue::Depth(depth, stencil) => {
                    clear_values.push(VkClearValue::depth_stencil(VkClearDepthStencilValue {
                        depth,
                        stencil,
                    }))
                }
            };

            // check for color attachment flag
            let (format, aspect) = if (target_info.usage & VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT) != 0
            {
                // add color attachment
                attachments.push(VkAttachmentDescription::new(
                    0,
                    target_info.format,
                    self.sample_count,
                    clear_operation,
                    store_operation,
                    VK_ATTACHMENT_LOAD_OP_DONT_CARE,
                    VK_ATTACHMENT_STORE_OP_DONT_CARE,
                    VK_IMAGE_LAYOUT_UNDEFINED,
                    if target_info.attach_sampler {
                        VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL
                    } else {
                        VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL
                    },
                ));

                // add color reference
                color_references.push(VkAttachmentReference {
                    attachment: attachment_count,
                    layout: VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
                });

                attachment_count += 1;

                // take format and aspect mask
                (target_info.format, VK_IMAGE_ASPECT_COLOR_BIT)
            // check for depth attachment flag
            } else if (target_info.usage & VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT) != 0 {
                // set depth reference
                if depth_reference.is_some() {
                    create_error!("more than one depth attachment is not allowed!");
                }

                // add depth attachment
                attachments.push(VkAttachmentDescription::new(
                    0,
                    target_info.format,
                    self.sample_count,
                    clear_operation,
                    store_operation,
                    VK_ATTACHMENT_LOAD_OP_DONT_CARE,
                    VK_ATTACHMENT_STORE_OP_DONT_CARE,
                    VK_IMAGE_LAYOUT_UNDEFINED,
                    VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                ));

                depth_reference = Some(VkAttachmentReference {
                    attachment: attachment_count,
                    layout: VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                });

                attachment_count += 1;

                // take format and aspect mask
                (target_info.format, VK_IMAGE_ASPECT_DEPTH_BIT)
            } else {
                // TODO: add more as required
                unimplemented!();
            };

            let mut image_builder = Image::empty(
                self.width,
                self.height,
                target_info.usage,
                self.sample_count,
            )
            .format(format)
            .aspect_mask(aspect)
            .memory_properties(0);

            if target_info.attach_sampler {
                image_builder = image_builder.nearest_sampler();
            }

            let image = image_builder.build(device, queue)?;

            match aspect {
                VK_IMAGE_ASPECT_DEPTH_BIT => {
                    Image::convert_layout(&image, VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL)?
                }
                VK_IMAGE_ASPECT_COLOR_BIT => {
                    Image::convert_layout(&image, VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL)?
                }
                _ => unimplemented!(),
            }

            images.push(image);
        }

        // add resolve target if possible
        if let Some(resolve_targets) = self.resolve_targets {
            // push color clear values
            clear_values.push(VkClearValue::color(VkClearColorValue::float32([
                0.0, 0.0, 0.0, 1.0,
            ])));

            // add color attachment
            attachments.push(VkAttachmentDescription::new(
                0,
                resolve_targets[0].vk_format(),
                VK_SAMPLE_COUNT_1_BIT,
                VK_ATTACHMENT_LOAD_OP_CLEAR,
                VK_ATTACHMENT_STORE_OP_STORE,
                VK_ATTACHMENT_LOAD_OP_DONT_CARE,
                VK_ATTACHMENT_STORE_OP_DONT_CARE,
                resolve_targets[0].image_layout()?,
                resolve_targets[0].image_layout()?,
            ));

            resolve_reference = Some(VkAttachmentReference {
                attachment: attachment_count,
                layout: resolve_targets[0].image_layout()?,
            });
        }

        let subpass_descriptions = [match resolve_reference {
            Some(resvole_ref) => VkSubpassDescription::new(
                0,
                &[],
                color_references.as_slice(),
                &[resvole_ref],
                match depth_reference {
                    Some(ref depth_ref) => Some(depth_ref),
                    None => None,
                },
                &[],
            ),
            None => VkSubpassDescription::new(
                0,
                &[],
                color_references.as_slice(),
                &[],
                match depth_reference {
                    Some(ref depth_ref) => Some(depth_ref),
                    None => None,
                },
                &[],
            ),
        }];

        let dependencies = if color_references.is_empty() {
            // assume, that when no color references are given,
            // we want to store the depth information for later
            if depth_reference.is_some() {
                for attachment in &mut attachments {
                    if attachment.format == VK_FORMAT_D16_UNORM {
                        attachment.storeOp = VK_ATTACHMENT_STORE_OP_STORE;
                        attachment.finalLayout = VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL;
                        break;
                    }
                }
            }

            [
                VkSubpassDependency::new(
                    VK_SUBPASS_EXTERNAL,
                    0,
                    VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT,
                    VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT,
                    0,
                    VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT
                        | VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT,
                    VK_DEPENDENCY_BY_REGION_BIT,
                ),
                VkSubpassDependency::new(
                    0,
                    VK_SUBPASS_EXTERNAL,
                    VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT,
                    VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT,
                    VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT
                        | VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT,
                    VK_ACCESS_SHADER_READ_BIT,
                    VK_DEPENDENCY_BY_REGION_BIT,
                ),
            ]
        } else {
            [
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
            ]
        };

        let renderpass = RenderPass::new(
            device.clone(),
            &subpass_descriptions,
            attachments.as_slice(),
            &dependencies,
        )?;

        Ok((renderpass, images, clear_values))
    }
}
