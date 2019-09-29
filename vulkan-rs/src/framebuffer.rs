use utilities::prelude::*;

use crate::impl_vk_handle;
use crate::prelude::*;

use std::sync::Arc;

pub struct FramebufferBuilder<'a> {
    render_pass: Option<&'a Arc<RenderPass>>,
    attachments: Vec<&'a Arc<Image>>,
    width: u32,
    height: u32,
    layers: u32,
}

impl<'a> FramebufferBuilder<'a> {
    pub fn set_render_pass(mut self, render_pass: &'a Arc<RenderPass>) -> Self {
        self.render_pass = Some(render_pass);

        self
    }

    pub fn add_attachment(mut self, image: &'a Arc<Image>) -> Self {
        self.attachments.push(image);

        self
    }

    pub fn set_width(mut self, width: u32) -> Self {
        self.width = width;

        self
    }

    pub fn set_height(mut self, height: u32) -> Self {
        self.height = height;

        self
    }

    pub fn set_layer_count(mut self, layers: u32) -> Self {
        self.layers = layers;

        self
    }

    pub fn build(mut self, device: Arc<Device>) -> VerboseResult<Arc<Framebuffer>> {
        if self.attachments.is_empty() {
            create_error!("no attachments added!");
        }

        // if width or height are not set, use first attachment as reference
        // may not work, if images have different sizes
        if self.width == 0 || self.height == 0 {
            self.width = self.attachments[0].width();
            self.height = self.attachments[0].height();
        }

        let mut image_views = Vec::with_capacity(self.attachments.len());
        let mut images = Vec::with_capacity(self.attachments.len());

        for attachment in self.attachments {
            image_views.push(attachment.vk_handle());
            images.push(attachment.clone());
        }

        let framebuffer_ci = VkFramebufferCreateInfo::new(
            VK_FRAMEBUFFER_CREATE_NULL_BIT,
            match self.render_pass {
                Some(render_pass) => render_pass.vk_handle(),
                None => create_error!("no render pass set!"),
            },
            &image_views,
            self.width,
            self.height,
            self.layers,
        );

        let framebuffer = device.create_framebuffer(&framebuffer_ci)?;

        Ok(Arc::new(Framebuffer {
            device,
            framebuffer,
            images,

            width: self.width,
            height: self.height,
        }))
    }
}

#[derive(Debug)]
pub struct Framebuffer {
    device: Arc<Device>,
    framebuffer: VkFramebuffer,
    images: Vec<Arc<Image>>,

    width: u32,
    height: u32,
}

impl Framebuffer {
    pub fn new<'a>() -> FramebufferBuilder<'a> {
        FramebufferBuilder {
            render_pass: None,
            attachments: Vec::new(),
            width: 0,
            height: 0,
            layers: 1,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn attachments(&self) -> &[Arc<Image>] {
        &self.images
    }
}

impl_vk_handle!(Framebuffer, VkFramebuffer, framebuffer);

impl Drop for Framebuffer {
    fn drop(&mut self) {
        self.device.destroy_framebuffer(self.framebuffer);
    }
}
