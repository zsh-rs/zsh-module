use std::ffi::CStr;

use crate::lifecycle::ModuleState;

use crate::flags::Flags;
use crate::types::result::Result;
use crate::types::ptr_iter::PointerIter;


type BuiltinFn<S> = fn(&S, &CStr, &[&CStr], &Flags) -> Result<()>;
type BuiltinFnMut<S> = fn(&mut S, &CStr, &[&CStr], &Flags) -> Result<()>;

type MathFn<S> = fn(&mut S) -> Result<()>;


pub trait Features {
    fn builtin(
        &self,
        func: BuiltinFn<Self>,
        name: *mut std::ffi::c_char,
        args: *mut *mut std::ffi::c_char,
        opts: *mut zsh::options,
    ) -> Result<()> {
        let name = unsafe { CStr::from_ptr(name) };
        let args: Vec<&CStr> = args
            .ptr_iter()
            .map(|ptr| unsafe { CStr::from_ptr(ptr) })
            .collect();
        let opts: Flags = opts.into();

        func(self, name, &args, &opts)
    }

    fn builtin_mut(
        &mut self,
        func: BuiltinFnMut<Self>,
        name: *mut std::ffi::c_char,
        args: *mut *mut std::ffi::c_char,
        opts: *mut zsh::options,
    ) -> Result<()> {
        let name = unsafe { CStr::from_ptr(name) };
        let args: Vec<&CStr> = args
            .ptr_iter()
            .map(|ptr| unsafe { CStr::from_ptr(ptr) })
            .collect();
        let opts: Flags = opts.into();

        func(self, name, &args, &opts)
    }

    fn mathfunc(
        &mut self,
        func: MathFn<Self>,
        argv1: *mut std::ffi::c_char,
        argv2: i32,
        argv3: *mut zsh::mnumber,
        argv4: i32,
    ) -> Result<()> {
        todo!("[mathfunc] {:?} {:?} {:?} {:?} {:?}", func, argv1, argv2, argv3, argv4)
    }

    // type ParamGetArgs = (*mut zsh::param,);
    // type ParamSetArgs = (*mut zsh::param, *mut c_char);
}

impl<M: ModuleState> Features for M {}
