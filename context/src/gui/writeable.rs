//! `Writeable` is a property to change text of an item

use crate::gui::textable::Textable;
use crate::prelude::*;

use std::sync::Arc;

pub trait ModifyText {
    fn set_text(&self, text: String) -> VerboseResult<()>;
    fn add_letter(&self, letter: char) -> VerboseResult<()>;
    fn remove_last(&self) -> VerboseResult<()>;
}

/// `Writeable` gives the ability to modify the text inside an `Textable`
pub struct Writeable {
    gui_handler: Arc<GuiHandler>,

    textable: Arc<dyn ModifyText>,
}

impl Writeable {
    /// Factory method for `Writeable`, returns `Arc<Writeable>`.
    ///
    /// # Arguments
    ///
    /// * `textable` is a `Arc<Textable>` instance
    pub fn new(
        gui_handler: Arc<GuiHandler>,
        textable: Arc<Textable>,
    ) -> VerboseResult<Arc<Writeable>> {
        let writeable = Arc::new(Writeable {
            gui_handler,

            textable: textable.clone() as Arc<dyn ModifyText>,
        });

        Self::add(&writeable)?;

        Ok(writeable)
    }

    /// Add method
    ///
    /// # Arguments
    ///
    /// * `writeable` is a `&Arc<Writeable>` instance that is going to be added
    pub fn add(writeable: &Arc<Writeable>) -> VerboseResult<()> {
        writeable.gui_handler.add_writeable(writeable)?;
        Ok(())
    }

    /// Delete method, has to be explicitly called, otherwise it will remain in memory.
    ///
    /// # Arguments
    ///
    /// * `writeable` is a `&Arc<Writeable>` instance that is going to be deleted
    pub fn delete(writeable: &Arc<Writeable>) -> VerboseResult<()> {
        writeable.gui_handler.delete_writeable(writeable)?;
        Ok(())
    }

    /// Replaces the text of `Textable`
    ///
    /// # Arguments
    ///
    /// * `text` that replaces the current text
    pub fn set_text(&self, text: String) -> VerboseResult<()> {
        self.textable.set_text(text)
    }

    /// Adds the letter to Text
    ///
    /// # Arguments
    ///
    /// * `letter` the letter thats going to be added
    pub fn add_letter(&self, letter: char) -> VerboseResult<()> {
        self.textable.add_letter(letter)
    }

    /// Removes the last letter
    pub fn remove_last(&self) -> VerboseResult<()> {
        self.textable.remove_last()
    }
}
