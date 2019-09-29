//! `Clickable` is a property to click an item per mouse

use crate::prelude::*;

use crate::gui::executable::Executable;
use crate::gui::frameable::Frameable;

use std::cell::{Cell, RefCell};
use std::sync::Arc;

/// `Clickable` gives the ability to execute a closure when clicked
pub struct Clickable {
    gui_handler: Arc<GuiHandler>,

    frameable: Arc<Frameable>,

    clicked: Cell<bool>,

    executable: Arc<Executable>,
    clicked_changed_callback: RefCell<Option<Box<dyn Fn() -> ()>>>,
}

impl Clickable {
    /// Factory method for `Clickable`, returns `Arc<Clickable>`
    ///
    /// # Arguments
    ///
    /// * `frameable` is a `Arc<Framable>` instance
    /// * `executable` is a `Arc<Exectuable>` instance
    pub fn new(
        gui_handler: Arc<GuiHandler>,
        frameable: Arc<Frameable>,
        executable: Arc<Executable>,
    ) -> VerboseResult<Arc<Clickable>> {
        let clickable = Arc::new(Clickable {
            gui_handler,

            frameable: frameable.clone(),

            clicked: Cell::new(false),
            executable,
            clicked_changed_callback: RefCell::new(None),
        });

        Self::add(&clickable)?;

        Ok(clickable)
    }

    /// Add method
    ///
    /// # Arguments
    ///
    /// * `clickable` is a `&Arc<Clickable>` instance that is going to be added
    pub fn add(clickable: &Arc<Clickable>) -> VerboseResult<()> {
        clickable.gui_handler.add_clickable(clickable)?;

        Ok(())
    }

    /// Delete method, has to be explicitly called, otherwise it will remain in memory
    ///
    /// # Arguments
    ///
    /// * `clickable` is a `&Arc<Clickable>` instance that is going to be deleted
    pub fn delete(clickable: &Arc<Clickable>) -> VerboseResult<()> {
        clickable.gui_handler.delete_clickable(&clickable)?;
        clickable.set_clicked(false)?;

        Ok(())
    }

    pub fn is_pressed(&self, x: u32, y: u32) -> bool {
        (x as i32 > self.frameable.left())
            && (x < self.frameable.right())
            && (y as i32 > self.frameable.top())
            && (y < self.frameable.bottom())
    }

    /// Callback when the clicked status of `self` has changed
    ///
    /// # Arguments
    ///
    /// * `clicked_changed_callback` is a `Option<Box<Fn() -> ()>>`
    pub fn set_clicked_changed_callback(
        &self,
        clicked_changed_callback: Option<Box<dyn Fn() -> ()>>,
    ) {
        self.clicked_changed_callback
            .replace(clicked_changed_callback);
    }

    pub fn set_clicked(&self, clicked: bool) -> VerboseResult<bool> {
        if self.clicked.get() != clicked {
            self.clicked.set(clicked);

            match self.clicked_changed_callback.try_borrow()?.as_ref() {
                Some(ref clicked_changed_callback) => (clicked_changed_callback)(),
                None => (),
            }

            if self.clicked.get() {
                self.executable.execute()?;
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Get current `clicked` status, returns `bool`
    pub fn clicked(&self) -> bool {
        self.clicked.get()
    }
}
