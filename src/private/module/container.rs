use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, Ordering};

use super::panic::panic_boundary;
use crate::private::error::{ZError, ZResult};
use crate::types::result::Result;

/// Global container for the module, statically allocated during compile time
///
pub struct Container<M: Default> {
    data: RefCell<Option<Box<M>>>,
    panicked: AtomicBool,
}

impl<S: Default> Container<S> {
    pub const fn new() -> Self {
        Self {
            data: RefCell::new(None),
            panicked: AtomicBool::new(false),
        }
    }

    pub fn init(&self) {
        self.data.borrow_mut().get_or_insert_default();
    }

    pub fn drop_data(&self) {
        let _ = self.data.borrow_mut().take();
    }

    pub fn with_state<F>(&self, cb: F) -> Option<i32>
    where
        F: FnOnce(&mut S) -> Result<()> + std::panic::UnwindSafe,
    {
        if self.panicked.load(Ordering::Acquire) {
            return None;
        }

        panic_boundary(|| -> i32 {
            let res: ZResult<()> = cb(self
                .data
                .borrow_mut()
                .as_mut()
                .expect("Module state is uninitialized"))
            .map_err(|e| ZError::CustomMessage(e.to_string()))
            .into();

            res.safe_unwrap()
        })
        .or_else(|| {
            self.panicked.store(true, Ordering::Release);
            None
        })
    }
}

unsafe impl<S: Default> Send for Container<S> {}
unsafe impl<S: Default> Sync for Container<S> {}

