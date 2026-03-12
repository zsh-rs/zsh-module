use std::ffi::{CStr, c_char};
use std::rc::Rc;
use zsh_sys as zsys;

use crate::ToCString;
use crate::module::LoadableModuleState;
use crate::options::Options;
use crate::types::result::Result;
use crate::types::string_array::PointerIter;

use super::registry::Registry;
use super::traits::{FeatureBuilder, FeatureDispatch};
use super::trampolines::builtin_trampoline;

pub(super) type BuiltinCommand<S> = fn(&mut S, &CStr, &[&CStr], &Options, i32) -> Result<()>;

pub struct BuiltinBuilder<S: LoadableModuleState> {
    pub(super) handler: BuiltinCommand<S>,
    pub(super) name: Rc<CStr>,
    pub(super) flags: Option<Rc<CStr>>,
    minargs: i32,
    maxargs: i32,
}

impl<S: LoadableModuleState> BuiltinBuilder<S> {
    /// Creates a command builtin.
    ///
    /// By default, the builtin can take any amount of arguments (minargs and maxargs are 0 and
    /// [`None`], respectively) and no flags.
    pub fn from(callback: BuiltinCommand<S>, name: &str) -> Self {
        Self {
            minargs: 0,
            maxargs: -1,
            flags: None,
            name: name.into_cstr().into(),
            handler: callback,
        }
    }

    /// Sets the minimum amount of arguments allowed by the builtin
    pub fn minargs(&mut self, value: i32) -> &Self {
        self.minargs = value;
        self
    }
    /// Sets the maximum amount of arguments allowed by the builtin
    pub fn maxargs(&mut self, value: Option<u32>) -> &Self {
        self.maxargs = value.map(|i| i as i32).unwrap_or(-1);
        self
    }
    /// Sets flags recognized by the builtin
    pub fn flags(&mut self, value: &str) -> &Self {
        self.flags = Some(value.into_cstr().into());
        self
    }
}

impl<S: LoadableModuleState> FeatureBuilder for BuiltinBuilder<S> {
    type FeatureType = zsys::builtin;

    fn strings(&self) -> Vec<Rc<CStr>> {
        let mut strings = vec![self.name.clone()];
        if let Some(ref flags) = self.flags {
            strings.push(flags.clone());
        }
        strings
    }

    fn build(&self, feature_id: i32) -> Self::FeatureType {
        let flags_ptr = match self.flags {
            Some(ref flags) => flags.as_ptr(),
            None => std::ptr::null(),
        };

        Self::FeatureType {
            node: zsys::hashnode {
                next: std::ptr::null_mut(),
                nam: self.name.clone().as_ptr() as *mut _,
                // !TODO: add flags param
                flags: 0,
            },
            handlerfunc: Some(builtin_trampoline),
            minargs: self.minargs,
            maxargs: self.maxargs,
            funcid: feature_id,
            optstr: flags_ptr as *mut i8,
            defopts: std::ptr::null_mut(),
        }
    }
}

pub struct BuiltinDispatch<'a, M: LoadableModuleState> {
    handler: Option<BuiltinCommand<M>>,
    pub name: &'a CStr,      // *mut c_char -> &'a str
    pub args: Vec<&'a CStr>, // *mut *mut c_char -> Vec<&'a str> -> &'a [&'a str]
    pub opts: &'a Options,   // *mut options -> &mut options
    pub func_id: i32,
}

type BuiltinArgs = (*mut c_char, *mut *mut c_char, *mut zsys::options, i32);

impl<'a, M: LoadableModuleState> From<BuiltinArgs> for BuiltinDispatch<'a, M> {
    fn from(value: BuiltinArgs) -> Self {
        let (name, args, opts, id) = value;
        Self {
            handler: None,
            name: unsafe { CStr::from_ptr(name) },
            args: args
                .ptr_iter()
                .map(|ptr| unsafe { CStr::from_ptr(ptr) })
                .collect(),
            opts: unsafe { &*opts },
            func_id: id,
        }
    }
}

impl<'a, M: LoadableModuleState> FeatureDispatch<M> for BuiltinDispatch<'a, M> {
    type FeatureOutput = ();

    fn get_handler(&mut self, registry: &Registry<M>) -> &Self {
        self.handler = self
            .handler
            .or(registry.builtin_handlers.get(self.name).cloned());
        self
    }

    fn execute(&self, state: &mut M) -> Result<Self::FeatureOutput> {
        match self.handler {
            Some(handler) => handler(state, self.name, &self.args, self.opts, self.func_id),
            None => Err(crate::ZError::HandlerNotFound(self.name.to_string_lossy().into()).into()),
        }
    }
}
