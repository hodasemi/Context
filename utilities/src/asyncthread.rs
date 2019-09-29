//! Asynchronous thread to check if result is returned (non-blocking)

use std::cell::Cell;
use std::sync::mpsc;
use std::thread;

/// Asynchronous thread handle
pub struct AsyncThread<T: Copy + Send + 'static> {
    receiver: mpsc::Receiver<T>,
    result: Cell<Option<T>>,
}

impl<T: Copy + Send + 'static> AsyncThread<T> {
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
            result: Cell::new(None),
        }
    }

    /// Checks if the thread is already returned
    /// Returns the Some(result) if the thread has finished its work,
    /// otherwise None
    pub fn check(&self) -> Option<T> {
        if self.result.get().is_none() {
            match self.receiver.try_recv() {
                Ok(result) => {
                    self.result.set(Some(result));
                    return self.result.get();
                }
                Err(_) => return None,
            }
        }

        self.result.get()
    }
}
