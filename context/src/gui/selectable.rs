//! `Selectable` is a property to select an item per button

use crate::gui::executable::Executable;
use crate::prelude::*;

use std::cell::{Cell, RefCell};
use std::sync::Arc;

/// `Selectable` gives the ability to navigate per button or controller to
/// optionally adjacent neighbour Selectables and to execute a closure
/// when the current Selectable is pressed
pub struct Selectable {
    gui_handler: Arc<GuiHandler>,

    selected: Cell<bool>,

    east_neighbour: RefCell<Option<Arc<Selectable>>>,
    west_neighbour: RefCell<Option<Arc<Selectable>>>,
    north_neighbour: RefCell<Option<Arc<Selectable>>>,
    south_neighbour: RefCell<Option<Arc<Selectable>>>,

    executable: Arc<Executable>,
    selected_changed_callback: RefCell<Option<Box<dyn Fn() -> ()>>>,
}

impl Selectable {
    /// Factory method for `Selectable`, returns `Arc<Selectable>`.
    ///
    /// # Arguments
    ///
    /// * `executable` is a `Arc<Executable>` instance
    pub fn new(
        gui_handler: Arc<GuiHandler>,
        executable: Arc<Executable>,
    ) -> VerboseResult<Arc<Selectable>> {
        let selectable = Arc::new(Selectable {
            gui_handler,

            selected: Cell::new(false),

            east_neighbour: RefCell::new(None),
            west_neighbour: RefCell::new(None),
            north_neighbour: RefCell::new(None),
            south_neighbour: RefCell::new(None),

            executable,
            selected_changed_callback: RefCell::new(None),
        });

        Self::add(&selectable)?;

        Ok(selectable)
    }

    /// Add method, has to be explicitly called, otherwise it will remain in memory.
    ///
    /// # Arguments
    ///
    /// * `selectable` is a `&Arc<Selectable>` instance that is going to be added
    pub fn add(selectable: &Arc<Selectable>) -> VerboseResult<()> {
        selectable.gui_handler.add_selectable(selectable)?;
        Ok(())
    }

    /// Delete method, has to be explicitly called, otherwise it will remain in memory.
    ///
    /// # Arguments
    ///
    /// * `selectable` is a `&Arc<Selectable>` instance that is going to be deleted
    pub fn delete(selectable: &Arc<Selectable>) -> VerboseResult<()> {
        selectable.gui_handler.delete_selectable(selectable)?;
        selectable.set_selected(false)
    }

    /// Selects this `Selectable`
    ///
    /// # Argument
    ///
    /// * `selectable` is a `Arc<Selectable>` instance
    pub fn select(selectable: &Arc<Selectable>) -> VerboseResult<()> {
        selectable
            .gui_handler
            .set_selectable(Some(selectable.clone()))
    }

    /// Sets the callback, which is called when this `Selectable` is selected
    ///
    /// # Argument
    ///
    /// * `selected_changed_callback` is a `Option<Box<Fn() -> ()>>` closure
    pub fn set_selected_changed_callback(
        &self,
        selected_changed_callback: Option<Box<dyn Fn() -> ()>>,
    ) {
        self.selected_changed_callback
            .replace(selected_changed_callback);
    }

    /// Sets the value of selected and calls the callback if the value changed.
    /// Generally used by the `GuiHandler`.
    ///
    /// # Arguments
    ///
    /// * `selected` is the new selected state
    pub fn set_selected(&self, selected: bool) -> VerboseResult<()> {
        if self.selected.get() != selected {
            self.selected.set(selected);

            if let Some(selected_changed_callback) =
                self.selected_changed_callback.try_borrow()?.as_ref()
            {
                (selected_changed_callback)();
            }
        }

        Ok(())
    }

    /// Returns the selected state. Generally used by the `GuiHandler`.
    pub fn selected(&self) -> bool {
        self.selected.get()
    }

    /// Executes the `Executable`'s callback
    pub fn click_event(&self) -> VerboseResult<()> {
        self.executable.execute()
    }

    /// Returns the current east neighbour, if possible
    pub fn east_neighbour(&self) -> VerboseResult<Option<Arc<Selectable>>> {
        match self.east_neighbour.try_borrow()?.as_ref() {
            Some(neighbour) => Ok(Some(neighbour.clone())),
            None => Ok(None),
        }
    }

    /// Replaces the current east neighbour
    ///
    /// # Arguments
    ///
    /// * `selectable` the new east neighbour
    pub fn set_east_neighbour(&self, selectable: Option<Arc<Selectable>>) -> VerboseResult<()> {
        *self.east_neighbour.try_borrow_mut()? = selectable;

        Ok(())
    }

    /// Returns the current west neighbour, if possible
    pub fn west_neighbour(&self) -> VerboseResult<Option<Arc<Selectable>>> {
        match self.west_neighbour.try_borrow()?.as_ref() {
            Some(neighbour) => Ok(Some(neighbour.clone())),
            None => Ok(None),
        }
    }

    /// Replaces the current west neighbour
    ///
    /// # Arguments
    ///
    /// * `selectable` the new west neighbour
    pub fn set_west_neighbour(&self, selectable: Option<Arc<Selectable>>) -> VerboseResult<()> {
        *self.west_neighbour.try_borrow_mut()? = selectable;

        Ok(())
    }

    /// Returns the current north neighbour, if possible
    pub fn north_neighbour(&self) -> VerboseResult<Option<Arc<Selectable>>> {
        match self.north_neighbour.try_borrow()?.as_ref() {
            Some(neighbour) => Ok(Some(neighbour.clone())),
            None => Ok(None),
        }
    }

    /// Replaces the current north neighbour
    ///
    /// # Argumnents
    ///
    /// * `selectable` the new north neighbour
    pub fn set_north_neighbour(&self, selectable: Option<Arc<Selectable>>) -> VerboseResult<()> {
        *self.north_neighbour.try_borrow_mut()? = selectable;

        Ok(())
    }

    /// Returns the current south neighbour, if possible
    pub fn south_neighbour(&self) -> VerboseResult<Option<Arc<Selectable>>> {
        match self.south_neighbour.try_borrow()?.as_ref() {
            Some(neighbour) => Ok(Some(neighbour.clone())),
            None => Ok(None),
        }
    }

    /// Replaces the current south neighbour
    ///
    /// # Arguments
    ///
    /// * `selectable` the new south neighbour
    pub fn set_south_neighbour(&self, selectable: Option<Arc<Selectable>>) -> VerboseResult<()> {
        *self.south_neighbour.try_borrow_mut()? = selectable;

        Ok(())
    }
}
