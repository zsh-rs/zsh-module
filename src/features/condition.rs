
use std::ffi::{CStr, c_char};

use crate::types::result::Result;
use crate::module::LoadableModuleState;
use crate::types::string_array::PointerIter;

use super::traits::FeatureDispatch;
use super::registry::Registry;




pub type ConditionHandler<S> = fn(&mut S) -> Result<()>;





type ConditionArgs = (*mut *mut c_char, i32);



pub(crate) struct CondtionDispatch<'a, M: LoadableModuleState> {
    handler: Option<ConditionHandler<M>>,
    // pub name: &'a CStr,      // *mut c_char -> &'a str
    pub args: Vec<&'a CStr>, // *mut *mut c_char -> Vec<&'a str> -> &'a [&'a str]
    // pub opts: &'a Options,   // *mut options -> &mut options
    pub id: i32,
}

impl<'a, M: LoadableModuleState> From<ConditionArgs> for CondtionDispatch<'a, M> {
    fn from(value: ConditionArgs) -> Self {
        let (args, id) = value;
        Self {
            handler: None,
            args: args
                .ptr_iter()
                .map(|ptr| unsafe { CStr::from_ptr(ptr) })
                .collect(),
            id,
        }
    }
}

impl<'a, M: LoadableModuleState> FeatureDispatch<M> for CondtionDispatch<'a, M> {
    type FeatureOutput = ();

    fn get_handler(&mut self, registry: &Registry<M>) -> &Self {
        self.handler = self
            .handler
            .or(registry.cond_handlers.get(self.id as usize).cloned());
        self
    }

    fn execute(&self, state: &mut M) -> Result<Self::FeatureOutput> {
        match self.handler {
            Some(handler) => handler(state),
            None => Err(crate::ZError::HandlerNotFound(format!("{}", self.id)).into()),
        }
    }
}