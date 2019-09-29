//! `Textable` is a property to display text on an item

use crate::gui::writeable::ModifyText;
use crate::prelude::*;

use super::texturedvertex::TexturedVertex;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;

/// `TextAlignment` is used to describe where the text of `Textable` is aligned to
#[derive(Clone, Copy)]
pub enum TextAlignment {
    Left,
    Right,
    Top,
    Bottom,
    Center,
}

/// `TextColor` describes the color of the text
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum TextColor {
    White,
    Black,
    Red,
    Blue,
    Green,
    Orange,
    Yellow,
}

impl TextColor {
    /// Returns a `Vector3<f32>` of the color
    pub fn vec3(&self) -> cgmath::Vector3<f32> {
        match *self {
            TextColor::White => cgmath::Vector3::new(1.0, 1.0, 1.0),
            TextColor::Black => cgmath::Vector3::new(0.0, 0.0, 0.0),
            TextColor::Red => cgmath::Vector3::new(1.0, 0.0, 0.0),
            TextColor::Blue => cgmath::Vector3::new(0.0, 0.0, 1.0),
            TextColor::Green => cgmath::Vector3::new(0.0, 1.0, 0.0),
            TextColor::Orange => cgmath::Vector3::new(1.0, 0.65, 0.0),
            TextColor::Yellow => cgmath::Vector3::new(1.0, 1.0, 0.0),
        }
    }
}

/// `Textable` gives the ability to display text inside an item
pub struct Textable {
    gui_handler: Arc<GuiHandler>,

    frameable: Arc<Frameable>,

    text_alignment: Cell<TextAlignment>,
    text: RefCell<String>,
    vertex_count: Cell<u32>,
    height_ratio: Cell<f32>,
    character_size: Cell<u32>,

    descriptor_set: RefCell<Arc<DescriptorSet>>,

    buffer: Rc<RefCell<Option<Arc<Buffer<TexturedVertex>>>>>,
}

impl Textable {
    /// Factory method for `Textable`, returns `Arc<Textable>`.
    ///
    /// # Arguments
    ///
    /// * `frameable` is a `Arc<Frameable>` instance
    /// * `text` the text to be displayed
    /// * `height_ratio` the ratio of the height in respect to the frameable height
    /// * `text_alignment` where the text is aligned to
    pub fn new(
        gui_handler: Arc<GuiHandler>,
        frameable: Arc<Frameable>,
        text: &str,
        height_ratio: f32,
        text_alignment: TextAlignment,
        text_color: TextColor,
    ) -> VerboseResult<Arc<Textable>> {
        let set = gui_handler.color_descriptor(text_color)?;

        let buffer = if text.is_empty() {
            None
        } else {
            Some(Self::create_text_buffer(
                gui_handler.device(),
                text.len() as u32 * 6,
            )?)
        };

        let textable = Arc::new(Textable {
            gui_handler,

            frameable: frameable.clone(),

            vertex_count: Cell::new(text.len() as u32 * 6),
            text_alignment: Cell::new(text_alignment),
            text: RefCell::new(text.to_string()),
            height_ratio: Cell::new(height_ratio),
            character_size: Cell::new(
                ((frameable.bottom() as i32 - frameable.top()) as f32 * height_ratio) as u32,
            ),

            descriptor_set: RefCell::new(set),

            buffer: Rc::new(RefCell::new(buffer)),
        });

        Self::add(&textable)?;

        let textable_clone = textable.clone();
        frameable.add_callback(Box::new(move || {
            check_and_return!(textable_clone.update_text());
        }))?;

        Ok(textable)
    }

    fn create_text_buffer(
        device: &Arc<Device>,
        letter_count: u32,
    ) -> VerboseResult<Arc<Buffer<TexturedVertex>>> {
        Buffer::new()
            .set_usage(VK_BUFFER_USAGE_VERTEX_BUFFER_BIT)
            .set_memory_properties(
                VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT,
            )
            .set_size(letter_count as VkDeviceSize)
            .build(device.clone())
    }

    /// Add method
    ///
    /// # Arguments
    ///
    /// * `textable` is a `&Arc<Textable>` instance that is going to be added
    pub fn add(textable: &Arc<Textable>) -> VerboseResult<()> {
        textable.gui_handler.add_textable(textable)?;
        Ok(())
    }

    /// Delete method, has to be explicitly called, otherwise it will remain in memory.
    ///
    /// # Arguments
    ///
    /// * `textable` is a `&Arc<Textable>` instance that is going to be deleted
    pub fn delete(textable: &Arc<Textable>) -> VerboseResult<()> {
        textable.gui_handler.delete_textable(textable)?;
        Ok(())
    }

    /// Changes text color
    ///
    /// # Arguments
    ///
    /// * `text_color` defines the color of the text
    pub fn set_text_color(&self, text_color: TextColor) -> VerboseResult<()> {
        let set = self.gui_handler.color_descriptor(text_color)?;

        *self.descriptor_set.try_borrow_mut()? = set;

        Ok(())
    }

    /// Sets the text alignment
    ///
    /// # Arguments
    ///
    /// * `text_alignment` where the text is aligned to
    pub fn set_text_alignment(&self, text_alignment: TextAlignment) -> VerboseResult<()> {
        self.text_alignment.set(text_alignment);
        self.update_text()
    }

    /// Returns the count of vertices described by the buffer
    pub fn vertex_count(&self) -> u32 {
        self.vertex_count.get()
    }

    /// Returns the internal vulkan buffer
    pub fn buffer(&self) -> VerboseResult<Option<Arc<Buffer<TexturedVertex>>>> {
        Ok(self.buffer.try_borrow()?.clone())
    }

    /// Returns the internal vulkan descriptor set
    pub fn descriptor_set(&self) -> VerboseResult<Arc<DescriptorSet>> {
        Ok(self.descriptor_set.try_borrow()?.clone())
    }

    /// Sets the text with height ratio
    ///
    /// # Arguments
    ///
    /// * `text` the text to be displayed
    /// * `height_ratio` the ratio of the height in respect to the frameable height
    pub fn set_text(&self, text: &str) -> VerboseResult<()> {
        if text.is_empty() {
            create_error!("empty text not allowed!");
        }

        self.vertex_count.set(text.len() as u32 * 6);
        *self.text.try_borrow_mut()? = text.to_string();
        self.update_text()
    }

    /// Sets the text size with the given height ratio
    ///
    /// # Arguments
    ///
    /// * `height_ratio` the ratio of the height in respect to the frameable height
    pub fn set_size(&self, height_ratio: f32) -> VerboseResult<()> {
        self.height_ratio.set(height_ratio);
        self.update_text()
    }

    /// Returns the text
    pub fn text(&self) -> VerboseResult<String> {
        Ok(self.text.try_borrow()?.clone())
    }

    fn calculate_text_size(&self) -> VerboseResult<()> {
        self.character_size.set(
            ((self.frameable.bottom() as i32 - self.frameable.top()) as f32
                * self.height_ratio.get()) as u32,
        );

        let width = self.frameable.right() as i32 - self.frameable.left();
        let text_len = self.text.try_borrow()?.len();

        let text_width = (self.character_size.get() as f32 * 0.5 * text_len as f32) as u32;

        if text_width > width as u32 {
            self.character_size.set(
                ((self.frameable.right() as i32 - self.frameable.left()) / text_len as i32) as u32
                    * 2,
            );
        }

        Ok(())
    }

    /// Updates the texts buffer
    pub fn update_text(&self) -> VerboseResult<()> {
        self.calculate_text_size()?;

        let (x, y) = self.calc_pos_from_alignment()?;

        self.create_buffer(x, y)?;

        Ok(())
    }

    fn calc_pos_from_alignment(&self) -> VerboseResult<(f32, f32)> {
        match self.text_alignment.get() {
            TextAlignment::Left => Ok((
                self.frameable.left() as f32,
                ((self.frameable.top() + self.frameable.bottom() as i32) as f32 * 0.5
                    - self.character_size.get() as f32 * 0.5),
            )),
            TextAlignment::Right => Ok((
                self.frameable.right() as f32
                    - (self.character_size.get() as f32
                        * 0.5
                        * self.text.try_borrow()?.len() as f32),
                ((self.frameable.top() + self.frameable.bottom() as i32) as f32
                    - self.character_size.get() as f32 * 0.5),
            )),
            TextAlignment::Top => Ok((
                ((self.frameable.left() + self.frameable.right() as i32) as f32 * 0.5
                    - (self.character_size.get() as f32
                        * 0.25
                        * self.text.try_borrow()?.len() as f32)),
                self.frameable.top() as f32,
            )),
            TextAlignment::Bottom => Ok((
                ((self.frameable.left() + self.frameable.right() as i32) as f32 * 0.5
                    - (self.character_size.get() as f32
                        * 0.25
                        * self.text.try_borrow()?.len() as f32)),
                (self.frameable.bottom() - self.character_size.get()) as f32,
            )),
            TextAlignment::Center => Ok((
                ((self.frameable.left() + self.frameable.right() as i32) as f32 * 0.5
                    - (self.character_size.get() as f32
                        * 0.25
                        * self.text.try_borrow()?.len() as f32)),
                ((self.frameable.top() + self.frameable.bottom() as i32) as f32 * 0.5
                    - self.character_size.get() as f32 * 0.5),
            )),
        }
    }

    fn create_buffer(&self, win_x: f32, win_y: f32) -> VerboseResult<()> {
        let weak_buffer = Rc::downgrade(&self.buffer);
        let text = self.text.try_borrow()?.clone();
        let character_size = self.character_size.get();
        let ortho = self.frameable.ortho();

        let device = self.gui_handler.device().clone();

        let async_buffer_creation = Box::new(move || {
            if let Some(buffer) = weak_buffer.upgrade() {
                let mut offset = 0.0;

                // variable to calculate letter position in bitmap font
                let letters_in_row = 16;
                let inverse_row = 0.0625;
                let inverse_col = 0.125;

                let letter_height = character_size as f32;
                let letter_width = letter_height * 0.5;

                let priority = 0.0;

                let mut buffer = buffer.try_borrow_mut()?;

                let text_buffer_len = text.len() as u32 * 6;

                match buffer.as_mut() {
                    Some(buf) => {
                        if buf.size() != text_buffer_len as VkDeviceSize {
                            *buffer = if text.is_empty() {
                                None
                            } else {
                                Some(Self::create_text_buffer(&device, text_buffer_len)?)
                            }
                        }
                    }
                    None => {
                        if !text.is_empty() {
                            *buffer = Some(Self::create_text_buffer(&device, text_buffer_len)?)
                        }
                    }
                }

                if let Some(buffer) = buffer.as_ref() {
                    let mut buffer_mapping = buffer.map_complete()?;

                    let mut i = 0;

                    for letter in text.chars() {
                        let mod_number = letter as u32 - 32;

                        // coordinates to describe letter position in bitmap font
                        let y = ((mod_number as f32) * inverse_row).floor();
                        let x = (mod_number - (y as u32 * letters_in_row)) as f32;

                        buffer_mapping[i] = TexturedVertex {
                            position: ortho
                                * cgmath::Vector4::new(
                                    win_x + offset,
                                    win_y + letter_height,
                                    priority,
                                    1.0,
                                ),
                            texture_coordinates: cgmath::Vector2::new(
                                inverse_row * x,
                                inverse_col + inverse_col * y,
                            ),
                        };

                        i += 1;

                        buffer_mapping[i] = TexturedVertex {
                            position: ortho
                                * cgmath::Vector4::new(
                                    win_x + offset + letter_width,
                                    win_y + letter_height,
                                    priority,
                                    1.0,
                                ),
                            texture_coordinates: cgmath::Vector2::new(
                                inverse_row + inverse_row * x,
                                inverse_col + inverse_col * y,
                            ),
                        };

                        i += 1;

                        buffer_mapping[i] = TexturedVertex {
                            position: ortho
                                * cgmath::Vector4::new(
                                    win_x + offset + letter_width,
                                    win_y,
                                    priority,
                                    1.0,
                                ),
                            texture_coordinates: cgmath::Vector2::new(
                                inverse_row + inverse_row * x,
                                inverse_col * y,
                            ),
                        };

                        i += 1;

                        buffer_mapping[i] = TexturedVertex {
                            position: ortho
                                * cgmath::Vector4::new(
                                    win_x + offset + letter_width,
                                    win_y,
                                    priority,
                                    1.0,
                                ),
                            texture_coordinates: cgmath::Vector2::new(
                                inverse_row + inverse_row * x,
                                inverse_col * y,
                            ),
                        };

                        i += 1;

                        buffer_mapping[i] = TexturedVertex {
                            position: ortho
                                * cgmath::Vector4::new(win_x + offset, win_y, priority, 1.0),
                            texture_coordinates: cgmath::Vector2::new(
                                inverse_row * x,
                                inverse_col * y,
                            ),
                        };

                        i += 1;

                        buffer_mapping[i] = TexturedVertex {
                            position: ortho
                                * cgmath::Vector4::new(
                                    win_x + offset,
                                    win_y + letter_height,
                                    priority,
                                    1.0,
                                ),
                            texture_coordinates: cgmath::Vector2::new(
                                inverse_row * x,
                                inverse_col + inverse_col * y,
                            ),
                        };

                        i += 1;

                        offset += letter_width;
                    }
                }
            }

            Ok(())
        });

        self.gui_handler
            .enqueue_text_update(async_buffer_creation)?;

        Ok(())
    }

    fn text_changed_through_write(&self) -> VerboseResult<()> {
        self.vertex_count
            .set(self.text.try_borrow()?.len() as u32 * 6);
        self.character_size.set(
            ((self.frameable.bottom() as i32 - self.frameable.top()) as f32
                * self.height_ratio.get()) as u32,
        );

        self.update_text()?;

        Ok(())
    }
}

impl ModifyText for Textable {
    fn set_text(&self, text: String) -> VerboseResult<()> {
        *self.text.try_borrow_mut()? = text;
        self.text_changed_through_write()
    }

    fn add_letter(&self, letter: char) -> VerboseResult<()> {
        self.text.try_borrow_mut()?.push(letter);
        self.text_changed_through_write()
    }

    fn remove_last(&self) -> VerboseResult<()> {
        self.text.try_borrow_mut()?.pop();
        self.text_changed_through_write()
    }
}
