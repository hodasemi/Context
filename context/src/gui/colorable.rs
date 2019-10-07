//! `Colorable` is a property to simply fill an area with a color

use crate::prelude::*;

use cgmath::{vec4, Vector4};

use std::cell::RefCell;
use std::sync::Arc;

/// `Colorable` gives the ability to fill an area with a color
pub struct Colorable {
    gui_handler: Arc<GuiHandler>,

    frameable: Arc<Frameable>,

    descriptor_set: RefCell<Arc<DescriptorSet>>,

    buffer: Arc<Buffer<Vector4<f32>>>,
}

impl Colorable {
    /// Factory method for `Colorable`, returns `Arc<Colorable>`.
    pub fn new(
        gui_handler: Arc<GuiHandler>,
        frameable: Arc<Frameable>,
        color: Color,
    ) -> VerboseResult<Arc<Colorable>> {
        let set = gui_handler.color_descriptor(color)?;

        let buffer = Buffer::new()
            .set_usage(VK_BUFFER_USAGE_VERTEX_BUFFER_BIT)
            .set_memory_properties(
                VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT,
            )
            .set_size(6)
            .build(gui_handler.device().clone())?;

        let colorable = Arc::new(Colorable {
            gui_handler,

            frameable: frameable.clone(),

            descriptor_set: RefCell::new(set),

            buffer,
        });

        Self::add(&colorable)?;

        let colorable_clone = colorable.clone();
        frameable.add_callback(Box::new(move || {
            check_and_return!(colorable_clone.update_frame());
        }))?;

        Ok(colorable)
    }

    /// Add method
    ///
    /// # Arguments
    ///
    /// * `colorable` is a `&Arc<Colorable>` instance that is going to be added
    pub fn add(colorable: &Arc<Colorable>) -> VerboseResult<()> {
        colorable.gui_handler.add_colorable(colorable)?;
        Ok(())
    }

    /// Delete method, has to be explicitly called, otherwise it will remain in memory.
    ///
    /// # Arguments
    ///
    /// * `colorable` is a `&Arc<Colorable>` instance that is going to be deleted
    pub fn delete(colorable: &Arc<Colorable>) -> VerboseResult<()> {
        colorable.gui_handler.delete_colorable(colorable)?;
        Ok(())
    }

    /// Changes text color
    ///
    /// # Arguments
    ///
    /// * `color` defines the color
    pub fn set_color(&self, color: Color) -> VerboseResult<()> {
        let set = self.gui_handler.color_descriptor(color)?;

        *self.descriptor_set.try_borrow_mut()? = set;

        Ok(())
    }

    /// Returns the internal vulkan buffer
    pub fn buffer(&self) -> &Arc<Buffer<Vector4<f32>>> {
        &self.buffer
    }

    /// Returns the internal vulkan descriptor set
    pub fn descriptor_set(&self) -> VerboseResult<Arc<DescriptorSet>> {
        Ok(self.descriptor_set.try_borrow()?.clone())
    }

    /// Update frame method if the original frame is invalidated
    pub fn update_frame(&self) -> VerboseResult<()> {
        let mut frame = self.buffer.map_complete()?;

        frame[0] = self.frameable.ortho()
            * vec4(
                self.frameable.left() as f32,
                self.frameable.bottom() as f32,
                0.0,
                1.0,
            );
        frame[1] = self.frameable.ortho()
            * vec4(
                self.frameable.right() as f32,
                self.frameable.bottom() as f32,
                0.0,
                1.0,
            );
        frame[2] = self.frameable.ortho()
            * vec4(
                self.frameable.right() as f32,
                self.frameable.top() as f32,
                0.0,
                1.0,
            );
        frame[3] = self.frameable.ortho()
            * vec4(
                self.frameable.right() as f32,
                self.frameable.top() as f32,
                0.0,
                1.0,
            );
        frame[4] = self.frameable.ortho()
            * vec4(
                self.frameable.left() as f32,
                self.frameable.top() as f32,
                0.0,
                1.0,
            );
        frame[5] = self.frameable.ortho()
            * vec4(
                self.frameable.left() as f32,
                self.frameable.bottom() as f32,
                0.0,
                1.0,
            );

        Ok(())
    }

    pub(crate) fn vertex_input_state() -> (
        VkPipelineVertexInputStateCreateInfo,
        Vec<VkVertexInputBindingDescription>,
        Vec<VkVertexInputAttributeDescription>,
    ) {
        let input_bindings = vec![VkVertexInputBindingDescription {
            binding: 0,
            stride: std::mem::size_of::<Vector4<f32>>() as u32,
            inputRate: VK_VERTEX_INPUT_RATE_VERTEX,
        }];

        let input_attributes = vec![VkVertexInputAttributeDescription {
            location: 0,
            binding: 0,
            format: VK_FORMAT_R32G32B32A32_SFLOAT,
            offset: 0,
        }];

        let input_state =
            VkPipelineVertexInputStateCreateInfo::new(0, &input_bindings, &input_attributes);

        (input_state, input_bindings, input_attributes)
    }
}
