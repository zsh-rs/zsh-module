

unsafe extern "C" {
    pub(super) unsafe fn builtin_trampoline(
        name: *mut std::ffi::c_char,
        args: *mut *mut std::ffi::c_char,
        opts: *mut zsh_sys::options,
        _arg: i32,
    ) -> i32;


    
}