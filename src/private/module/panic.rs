use std::ffi::CString;

use crate::private::module::name::module_name;

pub(super) fn panic_boundary<F, R>(cb: F) -> Option<R>
where
    F: FnOnce() -> R,
{
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| cb()));
    match res {
        Ok(ret) => Some(ret),
        Err(err) => {
            let err_message = if let Some(msg) = err.downcast_ref::<&str>() {
                format!("Panic: {}", msg)
            } else if let Some(msg) = err.downcast_ref::<String>() {
                format!("Panic: {}", msg)
            } else {
                format!("Panic: No additional information")
            };

            let err_cstr = CString::new(err_message).unwrap_or_else(|_| {
                CString::new("A panic occurred, but the panic message contained a null byte and could not be displayed.").unwrap()
            });

            // TODO: Figure out long term logging strategy
            unsafe { zsh::zerrnam(module_name().as_ptr() as _, err_cstr.as_ptr()) };

            None
        }
    }
}
