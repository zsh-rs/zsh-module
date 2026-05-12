use crate::private::module::name::module_name;

pub(super) fn panic_boundary<F, R>(cb: F) -> Option<R>
where
    F: FnOnce() -> R,
{
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| cb()));
    match res {
        Ok(ret) => Some(ret),
        Err(err) => {
            if let Some(msg) = err.downcast_ref::<&str>() {
                crate::error!("{:?} Panic: {}", module_name(), msg);
            } else if let Some(msg) = err.downcast_ref::<String>() {
                crate::error!("{:?} Panic: {}", module_name(), msg);
            } else {
                crate::error!("{:?} Panic: No additional information", module_name());
            }
            None
        }
    }
}