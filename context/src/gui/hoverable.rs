//! `Hoverable` is a property to hover over an item per mouse

use crate::gui::frameable::Frameable;
use crate::prelude::*;

use std::cell::{Cell, RefCell};
use std::sync::Arc;

/// `Hoverable` gives the ability to execute a closure when the mouse hovers over the item
pub struct Hoverable {
    gui_handler: Arc<GuiHandler>,

    frameable: Arc<Frameable>,

    hovered: Cell<bool>,

    hovered_changed_callback: RefCell<Option<Box<dyn Fn() -> ()>>>,
}

impl Hoverable {
    /// Factory method for `Hoverable`, returns `Arc<Hoverable>`.
    ///
    /// # Arguments
    ///
    /// * `frameable` is a `Arc<Frameable>` instance
    pub fn new(
        gui_handler: Arc<GuiHandler>,
        frameable: Arc<Frameable>,
    ) -> VerboseResult<Arc<Hoverable>> {
        let hoverable = Arc::new(Hoverable {
            gui_handler,
            frameable: frameable.clone(),
            hovered: Cell::new(false),
            hovered_changed_callback: RefCell::new(None),
        });

        Self::add(&hoverable)?;

        Ok(hoverable)
    }

    /// Add method
    ///
    /// # Arguments
    ///
    /// * `hoverable` is a `&Arc<Hoverable>` instance that is going to be added
    pub fn add(hoverable: &Arc<Hoverable>) -> VerboseResult<()> {
        hoverable.gui_handler.add_hoverable(hoverable)?;
        Ok(())
    }

    /// Delete method, has to be explicitly called, otherwise it will remain in memory.
    ///
    /// # Arguments
    ///
    /// * `hoverable` is a `&Arc<Hoverable>` instance that is going to be deleted
    pub fn delete(hoverable: &Arc<Hoverable>) -> VerboseResult<()> {
        hoverable.gui_handler.delete_hoverable(hoverable)?;
        hoverable.set_hovered(false)
    }

    /// Sets the callback which is called when the hovered state is changed
    ///
    /// # Arguments
    ///
    /// * `hovered_changed_callback` is a `Option<Box<Fn() -> ()>>` closure
    pub fn set_hovered_changed_callback(
        &self,
        hovered_changed_callback: Option<Box<dyn Fn() -> ()>>,
    ) {
        self.hovered_changed_callback
            .replace(hovered_changed_callback);
    }

    /// Returns true if x and y are inside the bounds of the given frameable,
    /// otherwise false. Generally used by the `GuiHandler`.
    ///
    /// # Arguments
    ///
    /// * `x` is the x value to check
    /// * `y` is the y value to check
    pub fn is_hovered(&self, x: u32, y: u32) -> bool {
        (x as i32 > self.frameable.left())
            && (x < self.frameable.right())
            && (y as i32 > self.frameable.top())
            && (y < self.frameable.bottom())
    }

    /// Sets the value of hovered and calls the callback if the value changed.
    /// Generally used by the `GuiHandler`.
    ///
    /// # Arguments
    ///
    /// * `hovered` is the new hovered state
    pub fn set_hovered(&self, hovered: bool) -> VerboseResult<()> {
        if self.hovered.get() != hovered {
            self.hovered.set(hovered);

            match self.hovered_changed_callback.try_borrow()?.as_ref() {
                Some(ref hovered_changed_callback) => (hovered_changed_callback)(),
                None => (),
            }
        }

        Ok(())
    }

    /// Returns the hovered state. Generally used by the `GuiHandler`.
    pub fn hovered(&self) -> bool {
        self.hovered.get()
    }
}
