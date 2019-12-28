//! Asynchronous thread to check if result is returned (non-blocking)

use crate::{
    create_error,
    errortype::{UtilError, VerboseResult},
};

use std::cell::RefCell;
use std::sync::mpsc;
use std::thread;

/// Asynchronous thread handle
pub struct AsyncThread<T: Send + 'static> {
    receiver: mpsc::Receiver<T>,
    result: RefCell<Option<T>>,
}

impl<T: Send + 'static> AsyncThread<T> {
    /// Spawns a thread
    ///
    /// # Arguments
    ///
    /// `f` is a function to be executed in a separate thread
    pub fn spawn<F>(f: F) -> AsyncThread<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
    {
        let (sender, receiver) = mpsc::channel();

        thread::spawn(move || {
            if sender.send((f)()).is_err() {
                panic!("sending failed!");
            }
        });

        AsyncThread {
            receiver,
            result: RefCell::new(None),
        }
    }

    /// Checks if the thread is already returned
    /// Returns the Some(result) if the thread has finished its work,
    /// otherwise None
    pub fn check(&self) -> VerboseResult<bool> {
        let mut result = self.result.try_borrow_mut()?;

        match result.as_ref() {
            Some(_) => Ok(true),
            None => match self.receiver.try_recv() {
                Ok(res) => {
                    *result = Some(res);

                    Ok(true)
                }
                Err(_) => Ok(false),
            },
        }
    }

    /// consumes the result
    pub fn take(&self) -> VerboseResult<T> {
        let mut result = self.result.try_borrow_mut()?;

        if result.is_some() {
            // actually safe to not panic, since we just checked
            let res = result.take().unwrap();

            Ok(res)
        } else {
            create_error!("no result present!")
        }
    }
}

impl<T: Send + 'static> AsyncThread<T>
where
    T: Clone,
{
    pub fn get(&self) -> VerboseResult<T> {
        let result = self.result.try_borrow()?;

        match result.as_ref() {
            Some(res) => Ok(res.clone()),
            None => create_error!("no result present!"),
        }
    }
}
