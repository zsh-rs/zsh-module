use std::rc::Rc;

use std::ffi::CStr;

use crate::module::LoadableModuleState;
use crate::types::result::Result;

use super::registry::Registry;

pub(super) trait FeatureBuilder {
    type FeatureType;

    /// ### Feature Strings
    ///
    /// Returns a [`Vec`] of [`Rc<CStr>`] that should be kept alive for the duration of the module's existence.
    ///
    ///
    /// This is for things like builtin flags, which are passed as C strings to ZSH and need to be kept alive for the duration of the module's existence.
    fn strings(&self) -> Vec<Rc<CStr>>;

    /// ### Builds the ZSH feature
    ///
    /// Takes the feature ID assigned by ZSH and returns the corresponding ZSH feature struct (e.g., `zsys::builtin` for builtins).
    fn build(&self, feature_id: i32) -> Self::FeatureType;
}

pub trait FeatureDispatch<M: LoadableModuleState> {
    type FeatureOutput;

    fn get_handler(&mut self, registry: &Registry<M>) -> &Self;

    fn execute(&self, state: &mut M) -> Result<Self::FeatureOutput>;
}
