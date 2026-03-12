use zsh_sys as zsys;

use crate::module::LoadableModuleState;

use super::context::FeatureContext;
use super::registry::Registry;
/// ### FeatureStore
///
/// The `FeatureStore` is a struct that holds the current state of the module's features and provides
#[derive(Default, Debug)]
pub(crate) struct FeatureStore<S: LoadableModuleState> {
    context: FeatureContext<S>,

    pub(crate) features: zsys::features,
}

impl<S: LoadableModuleState> FeatureStore<S> {
    pub(crate) fn registry(&self) -> &Registry<S> {
        &self.context.registry()
    }

    pub(crate) fn mutate_context<F>(&mut self, f: F)
    where
        F: FnOnce(&mut FeatureContext<S>),
    {
        f(&mut self.context);

        self.set_features()
    }

    fn set_features(&mut self) {
        if !self.context.builtins.is_empty() {
            self.features.bn_list = self.context.builtins.as_mut_ptr();
            self.features.bn_size = self.context.builtins.len() as i32;
        }
        if !self.context.conditions.is_empty() {
            self.features.cd_list = self.context.conditions.as_mut_ptr();
            self.features.cd_size = self.context.conditions.len() as i32;
        }
        if !self.context.mathfuncs.is_empty() {
            self.features.mf_list = self.context.mathfuncs.as_mut_ptr();
            self.features.mf_size = self.context.mathfuncs.len() as i32;
        }
        if !self.context.parameters.is_empty() {
            self.features.pd_list = self.context.parameters.as_mut_ptr();
            self.features.pd_size = self.context.parameters.len() as i32;
        }
    }
}
