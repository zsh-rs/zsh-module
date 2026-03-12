use super::entrypoints::LoadableModuleState;

use super::container::ModuleContainer;

pub fn panic_boundary<F, R, M>(module: &ModuleContainer<M>, cb: F) -> Option<R>
where
    F: FnOnce() -> R + std::panic::UnwindSafe,
    M: LoadableModuleState,
{
    let res = std::panic::catch_unwind(|| cb());
    match res {
        Ok(ret) => Some(ret),
        Err(err) => {
            module.panic();

            if let Some(msg) = err.downcast_ref::<&str>() {
                crate::error!("{:?} Panic: {}", module.name, msg);
            } else if let Some(msg) = err.downcast_ref::<String>() {
                crate::error!("{:?} Panic: {}", module.name, msg);
            } else {
                crate::error!("{:?} Panic: No additional information", module.name);
            }
            None
        }
    }
}

pub trait SafeUnwrap {
    fn safe_unwrap<M>(self, module: &ModuleContainer<M>) -> i32
    where
        M: LoadableModuleState;
}

impl SafeUnwrap for crate::ZResult<()> {
    fn safe_unwrap<M>(self, module: &ModuleContainer<M>) -> i32
    where
        M: LoadableModuleState,
    {
        match self {
            Ok(_) => 0,
            Err(e) => {
                crate::log::error_named(module.name, e.to_string());
                e.exit_code() as i32
            }
        }
    }
}

impl SafeUnwrap for crate::ZResult<i32> {
    fn safe_unwrap<M>(self, module: &ModuleContainer<M>) -> i32
    where
        M: LoadableModuleState,
    {
        match self {
            Ok(i) => i,
            Err(e) => {
                crate::log::error_named(module.name, e.to_string());
                e.exit_code() as i32
            }
        }
    }
}
