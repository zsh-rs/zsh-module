use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::ZError;
use crate::module::data::ModuleData;

use super::entrypoints::LoadableModuleState;

#[doc(hidden)]
///
/// Global container for the module, statically allocated during compile time
pub struct ModuleContainer<M: LoadableModuleState> {
    pub module: Mutex<ModuleData<M>>,
    pub panicked: AtomicBool,
    pub name: &'static str,
}

unsafe impl<M: LoadableModuleState> Send for ModuleContainer<M> {}
unsafe impl<M: LoadableModuleState> Sync for ModuleContainer<M> {}

impl<S: LoadableModuleState> ModuleContainer<S> {
    #[inline(always)]
    pub fn from(name: &'static str) -> Self {
        Self {
            module: Mutex::default(),
            panicked: AtomicBool::new(false),
            name,
        }
    }

    #[inline(always)]
    pub const fn get_name(&self) -> &'static str {
        self.name
    }

    #[inline(always)]
    pub fn panicked(&self) -> bool {
        self.panicked.load(Ordering::Acquire)
    }

    #[inline(always)]
    pub fn panic(&self) {
        self.panicked.store(true, Ordering::Release);
    }

    pub fn module_operation<F, R>(&self, f: F) -> Result<R, ZError>
    where
        F: FnOnce(&mut ModuleData<S>) -> Result<R, ZError>,
    {
        if self.panicked() {
            return Err(ZError::Panicked(format!(
                "Module {} is in a panicked state!",
                self.name
            )));
        }

        self.module
            .lock()
            .map_err(|_| ZError::Panicked(format!("Module {} is poisoned!", self.name)))
            .and_then(|mut zmod| f(&mut *zmod))
    }
}
