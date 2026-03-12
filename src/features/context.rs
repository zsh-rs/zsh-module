use std::ffi::CStr;
use std::rc::Rc;
use zsh_ffi as zsys;

use crate::module::LoadableModuleState;

use super::traits::FeatureBuilder;
use super::builtin::BuiltinBuilder;
use super::registry::Registry;

#[derive(Default, Debug)]
pub struct FeatureContext<S: LoadableModuleState> {
    handle_registry: Registry<S>,

    /// ## C-String Pool
    /// C-string pool for ZSH to reference without worrying about lifetimes. This is for things like builtin flags, which are passed as C strings to ZSH and need to be kept alive for the duration of the module's existence.
    strings: Vec<Rc<CStr>>,

    /// ## Builtins
    /// ZSH Builtin, This is for things like
    /// ```sh
    /// my_builtin arg1 arg2
    /// ```
    pub(super) builtins: Vec<zsys::builtin>,
    /// ## Conditions
    /// These are for things like
    /// ```sh
    /// [[ -my_test foo ]]
    /// ```
    pub(super) conditions: Vec<zsys::conddef>,
    /// ## Math Funcs
    /// ```sh
    /// $(( myfunc(1) ))
    /// ```
    pub(super) mathfuncs: Vec<zsys::mathfunc>,

    /// ## Parameters
    /// These are variables. ZSH doesn't "call" them; it reads/writes them. You might need "getter/setter" callbacks if the parameter is dynamic."
    pub(super) parameters: Vec<zsys::paramdef>,
}

impl<S> FeatureContext<S>
where
    S: LoadableModuleState,
{
    #[inline(always)]
    pub(super) fn registry(&self) -> &Registry<S> {
        &self.handle_registry
    }

    /// Registers a new builtin command
    ///
    /// See [`BuiltinBuilder`] for more details and examples on how to create a builtin command.
    #[inline(always)]
    pub fn add_builtin(&mut self, builder: BuiltinBuilder<S>) -> &mut Self {
        let idx = self.builtins.len();

        self.handle_registry
            .builtin_handlers
            .insert(builder.name.as_ref().into(), builder.handler);

        self.strings.extend(builder.strings());
        self.builtins.push(builder.build(idx as i32));

        self
    }

    /// Registers a new condition
    #[inline(always)]
    pub fn add_condition(&mut self) -> &Self {
        let _idx = self.conditions.len();
        todo!("Conditions are not yet implemented");
    }

    /// Registers a new math function
    #[inline(always)]
    pub fn add_mathfunc(&mut self) -> &Self {
        let _idx = self.mathfuncs.len();
        todo!("Math functions are not yet implemented");
    }

    /// Registers a new parameter
    #[inline(always)]
    pub fn add_parameter(&mut self) -> &Self {
        let _idx = self.parameters.len();
        todo!("Parameter hooks are not yet implemented");
    }
}
