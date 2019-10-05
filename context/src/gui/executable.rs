//! `Executable` is a property to execute a closure

use utilities::prelude::*;

use std::cell::RefCell;
use std::sync::Arc;

/// `Executable` holds a closure which can be executed
pub struct Executable {
    callback: RefCell<Option<Box<dyn Fn() -> VerboseResult<()>>>>,
}

impl Executable {
    /// Factory method for `Executable`, returns `Arc<Executable>`
    pub fn new() -> Arc<Executable> {
        Arc::new(Executable {
            callback: RefCell::new(None),
        })
    }

    /// Set callback closure
    ///
    /// # Arguments
    ///
    /// * `callback` is a `Option<Callback>` closure
    pub fn set_callback<F>(&self, callback: Option<F>) -> VerboseResult<()>
    where
        F: Fn() -> VerboseResult<()> + 'static,
    {
        let mut function = self.callback.try_borrow_mut()?;

        match callback {
            Some(f) => *function = Some(Box::new(f)),
            None => *function = None,
        }

        Ok(())
    }

    /// Execute the callback closure if possible
    pub fn execute(&self) -> VerboseResult<()> {
        if let Some(callback) = self.callback.try_borrow()?.as_ref() {
            (callback)()?;
        }

        Ok(())
    }
}
