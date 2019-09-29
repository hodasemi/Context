use std::cell::{Ref, RefCell};
use std::thread::{spawn, JoinHandle};

pub struct Future<T: Send + 'static> {
    thread: RefCell<Option<JoinHandle<T>>>,
    data: RefCell<Option<T>>,
}

impl<T: Send + 'static> Future<T> {
    pub fn new<F>(f: F) -> Future<T>
    where
        F: Fn() -> T,
        F: Send + 'static,
    {
        let thread = spawn(f);

        Future {
            thread: RefCell::new(Some(thread)),
            data: RefCell::new(None),
        }
    }

    pub fn data(&self) -> Ref<'_, Option<T>> {
        let mut thread_opt = self.thread.borrow_mut();

        if thread_opt.is_some() {
            let thread_tmp = thread_opt.take();
            let thread = thread_tmp.unwrap();
            if let Ok(data) = thread.join() {
                self.data.replace(Some(data));
            }
        }

        self.data.borrow()
    }
}
