//! `Displayable` is a property to display a background texture

use crate::prelude::*;

use super::texturedvertex::TexturedVertex;

use std::cell::RefCell;
use std::sync::Arc;

/// `Displayable` gives the ability to display a texture as background image for an item
pub struct Displayable {
    gui_handler: Arc<GuiHandler>,

    frameable: Arc<Frameable>,

    descriptor_set: RefCell<Arc<DescriptorSet>>,

    buffer: Arc<Buffer<TexturedVertex>>,
}

impl Displayable {
    /// Factory method for `Displayable`, returns `Arc<Displayable>`
    ///
    /// # Arguments
    ///
    /// * `frameable` is a `Arc<Frameable>` instance
    /// * `name` is the name for a png
    pub fn new(
        gui_handler: Arc<GuiHandler>,
        frameable: Arc<Frameable>,
        name: &str,
    ) -> VerboseResult<Arc<Displayable>> {
        let descriptor_set = gui_handler.image_descriptor(name)?;

        let buffer = Buffer::new()
            .set_usage(VK_BUFFER_USAGE_VERTEX_BUFFER_BIT)
            .set_memory_properties(
                VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT,
            )
            .set_size(6)
            .build(gui_handler.device().clone())?;

        let displayable = Arc::new(Displayable {
            gui_handler,
            frameable: frameable.clone(),
            descriptor_set: RefCell::new(descriptor_set),
            buffer,
        });

        Self::add(&displayable)?;

        Ok(displayable)
    }

    /// Add method
    ///
    /// # Arguments
    ///
    /// * `displayable` is a `&Arc<Displayable>` instance that is going to be added
    pub fn add(displayable: &Arc<Displayable>) -> VerboseResult<()> {
        let displayable_clone = displayable.clone();

        displayable.frameable.add_callback(
            "displayable",
            Box::new(move || displayable_clone.update_frame()),
        )?;

        displayable.gui_handler.add_displayable(displayable)?;
        Ok(())
    }

    /// Delete method, has to be explicitly called, otherwise it will remain in memory
    ///
    /// # Arguments
    ///
    /// * `displayable` is a `&Arc<Displayable>` instance that is going to be deleted
    pub fn delete(displayable: &Arc<Displayable>) -> VerboseResult<()> {
        displayable.frameable.remove_callback("displayable")?;

        displayable.gui_handler.delete_displayable(displayable)?;
        Ok(())
    }

    /// Replaces the current background image
    ///
    /// # Arguments
    ///
    /// * `name` is the name of the texture in `data/gui/` without `.png` prefix
    pub fn set_image(&self, name: &str) -> VerboseResult<()> {
        let descriptor_set = self.gui_handler.image_descriptor(name)?;

        *self.descriptor_set.try_borrow_mut()? = descriptor_set;

        Ok(())
    }

    /// Returns the internal vulkan buffer
    pub fn buffer(&self) -> &Arc<Buffer<TexturedVertex>> {
        &self.buffer
    }

    /// Returns the internal vulkan descriptor set
    pub fn descriptor_set(&self) -> VerboseResult<Arc<DescriptorSet>> {
        Ok(self.descriptor_set.try_borrow()?.clone())
    }

    /// Update frame method if the original frame is invalidated
    pub fn update_frame(&self) -> VerboseResult<()> {
        let mut frame = self.buffer.map_complete()?;

        frame[0].position = self.frameable.ortho()
            * cgmath::Vector4::new(
                self.frameable.left() as f32,
                self.frameable.bottom() as f32,
                0.0,
                1.0,
            );
        frame[1].position = self.frameable.ortho()
            * cgmath::Vector4::new(
                self.frameable.right() as f32,
                self.frameable.bottom() as f32,
                0.0,
                1.0,
            );
        frame[2].position = self.frameable.ortho()
            * cgmath::Vector4::new(
                self.frameable.right() as f32,
                self.frameable.top() as f32,
                0.0,
                1.0,
            );
        frame[3].position = self.frameable.ortho()
            * cgmath::Vector4::new(
                self.frameable.right() as f32,
                self.frameable.top() as f32,
                0.0,
                1.0,
            );
        frame[4].position = self.frameable.ortho()
            * cgmath::Vector4::new(
                self.frameable.left() as f32,
                self.frameable.top() as f32,
                0.0,
                1.0,
            );
        frame[5].position = self.frameable.ortho()
            * cgmath::Vector4::new(
                self.frameable.left() as f32,
                self.frameable.bottom() as f32,
                0.0,
                1.0,
            );

        frame[0].texture_coordinates = cgmath::Vector2::new(0.0, 1.0);
        frame[1].texture_coordinates = cgmath::Vector2::new(1.0, 1.0);
        frame[2].texture_coordinates = cgmath::Vector2::new(1.0, 0.0);
        frame[3].texture_coordinates = cgmath::Vector2::new(1.0, 0.0);
        frame[4].texture_coordinates = cgmath::Vector2::new(0.0, 0.0);
        frame[5].texture_coordinates = cgmath::Vector2::new(0.0, 1.0);

        Ok(())
    }
}
