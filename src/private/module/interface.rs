use crate::lifecycle::ModuleState;
use crate::types::result::Result;

use super::container::Container;

/// TODO
pub type TrampolineCallback<S> = Box<dyn FnOnce(&mut S) -> Result<()> + std::panic::UnwindSafe>;

pub trait SizedModuleState: Sized + Default + std::fmt::Debug + ModuleState {}


pub trait ContainerHooks: Send + Sync {
    fn init(&self);
    fn drop_data(&self);
    fn with_state(&self, cb: TrampolineCallback<dyn ModuleState>) -> Option<i32>;
}

impl<S: SizedModuleState> ContainerHooks for Container<S> {
    #[inline(always)]
    fn init(&self) {
        Container::init(self)
    }

    #[inline(always)]
    fn drop_data(&self) {
        Container::drop_data(self)
    }

    #[inline(always)]
    fn with_state(&self, cb: TrampolineCallback<dyn ModuleState>) -> Option<i32> {
        Container::with_state(self, |state| cb(state))
    }
}
