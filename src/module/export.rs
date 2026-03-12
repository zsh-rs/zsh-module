#[macro_export]
macro_rules! zmodule {
    ($module_name:ident) => {
        use $crate::zsh_sys as zsys;

        use $crate::module::container::ModuleContainer;
        use $crate::module::data::ModuleData;
        use $crate::module::panic::SafeUnwrap;
        use $crate::module::panic::panic_boundary;
        use $crate::types::error::ZResult;

        use $crate::features::builtin::BuiltinDispatch;
        use ::std::ffi::{c_char, c_int};
        use ::std::{mem::MaybeUninit, panic::UnwindSafe};

        static mut MODULE_CONTAINER: MaybeUninit<ModuleContainer<$module_name>> = MaybeUninit::uninit();
        const MOD_NAME: &'static str = stringify!($module_name);

        fn use_module<F, T>(applicator: F) -> i32
        where
            ZResult<T>: SafeUnwrap,
            F: FnOnce(&mut ModuleData<$module_name>) -> ZResult<T> + UnwindSafe,
        {
            let container = unsafe { MODULE_CONTAINER.assume_init_ref() };
            panic_boundary(container, || {
                container
                    .module_operation(applicator)
                    .safe_unwrap(container)
            })
            .unwrap_or(65)
        }

        #[doc(hidden)]
        #[unsafe(no_mangle)]
        extern "C" fn builtin_trampoline(
            name: *mut c_char,
            args: *mut *mut c_char,
            opts: *mut zsys::options,
            id: i32,
        ) -> i32 {
            let mut args = BuiltinDispatch::from((name, args, opts, id));

            use_module(move |module| module.trampoline(&mut args))
        }

        #[doc(hidden)]
        #[unsafe(no_mangle)]
        extern "C" fn setup_(_: zsys::Module) -> c_int {
            unsafe { MODULE_CONTAINER.write(ModuleContainer::from(MOD_NAME)) };
            use_module(|module| module.register())
        }

        $crate::module_hooks! {
            fn features(features_ptr: *mut *mut *mut c_char) -> c_int;
            fn enables(enables_ptr: *mut *mut c_int) -> c_int;
            fn boot() -> c_int;
            fn cleanup() -> c_int;
        }

        #[doc(hidden)]
        #[unsafe(no_mangle)]
        extern "C" fn finish_(_: zsys::Module) -> c_int {
            unsafe { MODULE_CONTAINER.assume_init_drop() };
            0
        }
    };
}
