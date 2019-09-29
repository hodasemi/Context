//! `Iconizable` is a property to display a icon on an item

use crate::prelude::*;

use super::texturedvertex::TexturedVertex;

use std::cell::{Cell, RefCell};
use std::sync::Arc;

/// `Iconizable` gives the ability to display a icon as icon on an UI item
pub struct Iconizable {
    gui_handler: Arc<GuiHandler>,

    frameable: Arc<Frameable>,

    buffer: Arc<Buffer<TexturedVertex>>,
    icon: RefCell<Arc<Image>>,

    descriptor_set: Arc<DescriptorSet>,

    x: Cell<i32>,
    y: Cell<i32>,
    width: Cell<u32>,
    height: Cell<u32>,
    vertical_alignment: Cell<VerticalAlign>,
    horizontal_alignment: Cell<HorizontalAlign>,
}

impl Iconizable {
    /// Factory method for `Iconizable`, returns `Arc<Iconizable>`
    ///
    /// # Arguments
    ///
    /// * `frameable` is a `Arc<Frameable>` instance
    /// * `icon` is a reference to an `Arc<Image>`
    pub fn new(
        gui_handler: Arc<GuiHandler>,
        frameable: Arc<Frameable>,
        icon: &Arc<Image>,
    ) -> VerboseResult<Arc<Iconizable>> {
        let device = gui_handler.device();

        let desc_pool = DescriptorPool::new()
            .set_layout(gui_handler.icon_descriptor_layout().clone())
            .build(device.clone())?;

        let descriptor_set = DescriptorPool::prepare_set(&desc_pool).allocate()?;

        let buffer = Buffer::new()
            .set_usage(VK_BUFFER_USAGE_VERTEX_BUFFER_BIT)
            .set_memory_properties(
                VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT,
            )
            .set_size(6)
            .build(device.clone())?;

        let iconizable = Arc::new(Iconizable {
            gui_handler,

            frameable,

            buffer,
            icon: RefCell::new(icon.clone()),

            descriptor_set,

            x: Cell::new(0),
            y: Cell::new(0),
            width: Cell::new(0),
            height: Cell::new(0),
            vertical_alignment: Cell::new(VerticalAlign::Notset),
            horizontal_alignment: Cell::new(HorizontalAlign::Notset),
        });

        iconizable.set_icon(icon)?;
        Self::add(&iconizable)?;

        Ok(iconizable)
    }

    /// Add method
    ///
    /// # Arguments
    ///
    /// * `iconizable` is a `&Arc<Iconizable>` instance that is going to be added
    pub fn add(iconizable: &Arc<Iconizable>) -> VerboseResult<()> {
        iconizable.gui_handler.add_iconizable(iconizable)?;
        Ok(())
    }

    /// Delete method, has to be explicitly called, otherwise it will remain in memory
    ///
    /// # Arguments
    ///
    /// * `iconizable` is a `&Arc<Iconizable>` instance that is going to be deleted
    pub fn delete(iconizable: &Arc<Iconizable>) -> VerboseResult<()> {
        iconizable.gui_handler.delete_iconizable(iconizable)?;
        Ok(())
    }

    /// Sets the frame parameter for
    ///
    /// # Arguments
    ///
    /// * `x` x offset to the frameables x value
    /// * `y` y offset to the frameables y value
    /// * `width` width of the icon
    /// * `height` height if the icon
    /// * `vertical_alignment` where this icon is aligned to inside the frameable
    /// * `horizontal_alignment` where this icon is aligned to inside the frameable
    pub fn set_frame(
        &self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        vertical_alignment: VerticalAlign,
        horizontal_alignment: HorizontalAlign,
    ) {
        self.x.set(x);
        self.y.set(y);
        self.width.set(width);
        self.height.set(height);

        self.vertical_alignment.set(vertical_alignment);
        self.horizontal_alignment.set(horizontal_alignment);
    }

    /// Updates the frame
    pub fn update_frame(&self) -> VerboseResult<()> {
        if !self.frameable.is_framed() {
            create_error!("frameable is not framed yet".to_string());
        }

        let x_start = self.frameable.left();
        let x_end = self.frameable.right();
        let y_start = self.frameable.top();
        let y_end = self.frameable.bottom();

        let parent_width = x_end as i32 - x_start;
        let parent_height = y_end as i32 - y_start;

        let y_align = match self.vertical_alignment.get() {
            VerticalAlign::Top => 0,
            VerticalAlign::Middle => parent_height / 2,
            VerticalAlign::Bottom => parent_height,
            _ => create_error!("vertical alignment in frameable not set".to_string()),
        };

        let x_align = match self.horizontal_alignment.get() {
            HorizontalAlign::Left => 0,
            HorizontalAlign::Middle => parent_width / 2,
            HorizontalAlign::Right => parent_width,
            _ => create_error!("horizontal alignment in frameable not set".to_string()),
        };

        let left = x_align as i32 + self.x.get();
        let right = left + self.width.get() as i32;
        let top = y_align as i32 + self.y.get();
        let bottom = top + self.height.get() as i32;

        // bounds checking
        if left < 0 {
            create_error!(format!("left ({}) can't be below zero", left));
        }

        if right > parent_width {
            create_error!(format!(
                "right ({}) can't be above parent width ({})",
                right, parent_width
            ));
        }

        if top < 0 {
            create_error!(format!("top ({}) can't be below zero", top));
        }

        if bottom > parent_height {
            create_error!(format!(
                "bottom ({}) can't be above parent height ({})",
                bottom, parent_height
            ));
        }

        let ortho = self.frameable.ortho();

        let mut frame = self.buffer.map_complete()?;

        let abs_left: f32 = (left + x_start as i32) as f32;
        let abs_right: f32 = (right + x_start as i32) as f32;
        let abs_top: f32 = (top + y_start as i32) as f32;
        let abs_bottom: f32 = (bottom + y_start as i32) as f32;

        frame[0].position = ortho * cgmath::Vector4::new(abs_left, abs_bottom, 0.0, 1.0);
        frame[1].position = ortho * cgmath::Vector4::new(abs_right, abs_bottom, 0.0, 1.0);
        frame[2].position = ortho * cgmath::Vector4::new(abs_right, abs_top, 0.0, 1.0);
        frame[3].position = ortho * cgmath::Vector4::new(abs_right, abs_top, 0.0, 1.0);
        frame[4].position = ortho * cgmath::Vector4::new(abs_left, abs_top, 0.0, 1.0);
        frame[5].position = ortho * cgmath::Vector4::new(abs_left, abs_bottom, 0.0, 1.0);

        frame[0].texture_coordinates = cgmath::Vector2::new(0.0, 1.0);
        frame[1].texture_coordinates = cgmath::Vector2::new(1.0, 1.0);
        frame[2].texture_coordinates = cgmath::Vector2::new(1.0, 0.0);
        frame[3].texture_coordinates = cgmath::Vector2::new(1.0, 0.0);
        frame[4].texture_coordinates = cgmath::Vector2::new(0.0, 0.0);
        frame[5].texture_coordinates = cgmath::Vector2::new(0.0, 1.0);

        Ok(())
    }

    /// Replace the current icon with a new one
    ///
    /// # Arguments
    ///
    /// * `icon` the new icon
    pub fn set_icon(&self, icon: &Arc<Image>) -> VerboseResult<()> {
        *self.icon.try_borrow_mut()? = icon.clone();

        self.descriptor_set
            .update(&[DescriptorWrite::combined_samplers(0, &[icon])]);

        Ok(())
    }

    /// Returns the internal vulkan buffer
    pub fn buffer(&self) -> &Arc<Buffer<TexturedVertex>> {
        &self.buffer
    }

    /// Returns the internal descriptor set
    pub fn descriptor_set(&self) -> &Arc<DescriptorSet> {
        &self.descriptor_set
    }
}
