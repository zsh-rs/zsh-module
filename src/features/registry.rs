use std::{collections::HashMap, ffi::CString};

use crate::module::LoadableModuleState;

use super::builtin::BuiltinCommand;
use super::condition::ConditionHandler;
use super::math::MathHandler;


#[derive(Default, Debug)]
pub struct Registry<M: LoadableModuleState> {
    // ID-based dispatch (Conditions & Math)
    // Index = infid/id
    pub(super) cond_handlers: Vec<ConditionHandler<M>>,
    pub(super) math_handlers: Vec<MathHandler<M>>,

    // Name-based dispatch (Builtins & Parameters)
    pub(super) builtin_handlers: HashMap<CString, BuiltinCommand<M>>,
    // pub(super) param_handlers: HashMap<CString, ParamHandler<M>>,
}