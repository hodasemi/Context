//! `Frameable` is a property to frame an item

use crate::prelude::*;

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::sync::Arc;

use cgmath;

/// Describes the vertical alignment for a `Frameable`
#[derive(Copy, Clone, PartialEq)]
pub enum VerticalAlign {
    Notset,
    Top,
    Middle,
    Bottom,
}

/// Describes the horizontal alignment for a `Frameable`
#[derive(Copy, Clone, PartialEq)]
pub enum HorizontalAlign {
    Notset,
    Left,
    Middle,
    Right,
}

/// `Frameable` keeps track of the position and size of an item
/// and calls functions on resize to keep everything aligned correctly
pub struct Frameable {
    gui_handler: Arc<GuiHandler>,

    left: Cell<i32>,
    right: Cell<u32>,
    top: Cell<i32>,
    bottom: Cell<u32>,

    vertical_alignment: Cell<VerticalAlign>,
    horizontal_alignment: Cell<HorizontalAlign>,

    resize_callbacks: RefCell<HashMap<String, Box<dyn Fn() -> VerboseResult<()>>>>,

    x_off: Cell<i32>,
    y_off: Cell<i32>,
    w: Cell<u32>,
    h: Cell<u32>,

    framed: Cell<bool>,

    // old extent, needed for size check in enable
    window_width: Cell<u32>,
    window_height: Cell<u32>,
}

impl Frameable {
    /// Factory method for `Frameable`, returns `Arc<Frameable>`
    pub fn new(gui_handler: Arc<GuiHandler>) -> VerboseResult<Arc<Frameable>> {
        let frameable = Arc::new(Frameable {
            window_width: Cell::new(gui_handler.width()),
            window_height: Cell::new(gui_handler.height()),

            gui_handler,

            left: Cell::new(0),
            right: Cell::new(0),
            top: Cell::new(0),
            bottom: Cell::new(0),

            vertical_alignment: Cell::new(VerticalAlign::Notset),
            horizontal_alignment: Cell::new(HorizontalAlign::Notset),

            resize_callbacks: RefCell::new(HashMap::new()),

            x_off: Cell::new(0),
            y_off: Cell::new(0),
            w: Cell::new(0),
            h: Cell::new(0),

            framed: Cell::new(false),
        });

        Self::add(&frameable)?;

        Ok(frameable)
    }

    /// Add method
    ///
    /// # Arguments
    ///
    /// * `frameable` - is a `&Arc<Frameable>` instance that is going to be added
    pub fn add(frameable: &Arc<Frameable>) -> VerboseResult<()> {
        // check if window size is the same as last time
        if frameable.gui_handler.width() != frameable.window_width.get()
            || frameable.gui_handler.height() != frameable.window_height.get()
        {
            // update window size
            frameable.window_width.set(frameable.gui_handler.width());
            frameable.window_height.set(frameable.gui_handler.height());

            // force resize
            frameable.resize()?;
        }

        frameable.gui_handler.add_frameable(frameable)?;
        Ok(())
    }

    /// Delete method, has to be explicitly called, otherwise it will remain in memory
    ///
    /// # Arguments
    ///
    /// * `frameable` - is a `&Arc<Frameable>` instance that is going to be deleted
    pub fn delete(frameable: &Arc<Frameable>) -> VerboseResult<()> {
        frameable.gui_handler.delete_frameable(frameable)?;
        Ok(())
    }

    /// Method to set the frame to a certain position with certain
    /// width and a certain alignment
    pub fn set_frame(
        &self,
        x_off: i32,
        y_off: i32,
        w: u32,
        h: u32,
        vertical_align: VerticalAlign,
        horizontal_align: HorizontalAlign,
    ) {
        self.x_off.set(x_off);
        self.y_off.set(y_off);
        self.w.set(w);
        self.h.set(h);

        self.vertical_alignment.set(vertical_align);
        self.horizontal_alignment.set(horizontal_align);

        self.calculate_frame();

        self.framed.set(true);
    }

    /// Returns the left edge in pixels of this frameable,
    /// calculated from the left of the window
    pub fn left(&self) -> i32 {
        self.left.get()
    }

    /// Returns the right edge in pixels of this frameable,
    /// calculated from the left of the window
    pub fn right(&self) -> u32 {
        self.right.get()
    }

    /// Returns the top edge in pixels of this frameable,
    /// calculated from the top of the window
    pub fn top(&self) -> i32 {
        self.top.get()
    }

    /// Returns the bottom edge in pixels of this frameable,
    /// calculated from the top of the window
    pub fn bottom(&self) -> u32 {
        self.bottom.get()
    }

    /// Returns `true` if `set_frame` got called, otherwise `false`
    pub fn is_framed(&self) -> bool {
        self.framed.get()
    }

    /// Returns the ortho 4x4 matrix, which describes the window
    pub fn ortho(&self) -> cgmath::Matrix4<f32> {
        self.gui_handler.ortho()
    }

    /// Adds a callback closure which is executed on resize
    pub fn add_callback(
        &self,
        id: &str,
        callback: Box<dyn Fn() -> VerboseResult<()>>,
    ) -> VerboseResult<()> {
        if cfg!(debug_assertions) {
            if self.resize_callbacks.try_borrow()?.get(id).is_some() {
                create_error!(format!("could not add {} twice", id));
            }
        }

        self.resize_callbacks
            .try_borrow_mut()?
            .insert(id.to_string(), callback);

        Ok(())
    }

    pub fn remove_callback(&self, id: &str) -> VerboseResult<()> {
        self.resize_callbacks.try_borrow_mut()?.remove(id);

        Ok(())
    }

    // Returns vertical alignment of this frameable
    pub fn vertical_alignment(&self) -> VerticalAlign {
        self.vertical_alignment.get()
    }

    // Returns horizontal alignment of this frameable
    pub fn horizontal_alignment(&self) -> HorizontalAlign {
        self.horizontal_alignment.get()
    }

    fn calculate_frame(&self) {
        let width = self.gui_handler.width();
        let height = self.gui_handler.height();

        let y_align = match self.vertical_alignment.get() {
            VerticalAlign::Top => 0,
            VerticalAlign::Middle => height / 2,
            VerticalAlign::Bottom => height,
            _ => {
                if cfg!(debug_assertions) {
                    println!("vertical alignment in frameable not set");
                }

                height / 2
            }
        };

        let x_align = match self.horizontal_alignment.get() {
            HorizontalAlign::Left => 0,
            HorizontalAlign::Middle => width / 2,
            HorizontalAlign::Right => width,
            _ => {
                if cfg!(debug_assertions) {
                    println!("horizontal alignment in frameable not set");
                }

                width / 2
            }
        };

        let left = x_align as i32 + self.x_off.get();
        let right = left + self.w.get() as i32;
        let top = y_align as i32 + self.y_off.get();
        let bottom = top + self.h.get() as i32;

        if cfg!(debug_assertions) {
            if left < 0 {
                println!("left ({}) can't be below zero", left);
            }

            if right > self.gui_handler.width() as i32 {
                println!(
                    "right ({}) can't be above window width ({})",
                    right,
                    self.gui_handler.width()
                );
            }

            if top < 0 {
                println!("top ({}) can't be below zero", top);
            }

            if bottom > self.gui_handler.height() as i32 {
                println!(
                    "bottom ({}) can't be above window height ({})",
                    bottom,
                    self.gui_handler.height()
                );
            }
        }

        self.left.set(left);
        self.right.set(right as u32);
        self.top.set(top);
        self.bottom.set(bottom as u32);
    }

    pub fn resize(&self) -> VerboseResult<()> {
        self.calculate_frame();

        for (_, callback) in self.resize_callbacks.try_borrow_mut()?.iter() {
            callback()?;
        }

        Ok(())
    }
}
