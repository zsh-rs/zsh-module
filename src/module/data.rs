use std::ffi::{c_char, c_int};

use zsh_sys as zsys;

use super::entrypoints::LoadableModuleState;
use crate::features::{store::FeatureStore, traits::FeatureDispatch};
use crate::{ZError, ZResult};

/// Hooks into the Zsh module system and connects it to your `User Data`.
#[derive(Default, Debug)]
pub struct ModuleData<M: LoadableModuleState> {
    pub(crate) state: Box<M>,
    pub(crate) store: FeatureStore<M>,
}

impl<M: LoadableModuleState> ModuleData<M> {
    pub fn register(&mut self) -> ZResult<()> {
        self.store.mutate_context(|ctx| self.state.features(ctx));
        Ok(())
    }

    pub fn features(
        &mut self,
        zmod: zsys::Module,
        features_ptr: *mut *mut *mut c_char,
    ) -> ZResult<()> {
        unsafe {
            *features_ptr = zsys::featuresarray(zmod, &mut self.store.features);
        }
        Ok(())
    }

    pub fn enables(&mut self, zmod: zsys::Module, enables_ptr: *mut *mut c_int) -> ZResult<i32> {
        Ok(unsafe { zsys::handlefeatures(zmod, &mut self.store.features, enables_ptr) })
    }

    pub fn boot(&mut self, _: zsys::Module) -> ZResult<()> {
        // zsys::addwrapper() ?
        self.state
            .start()
            .map_err(|e| ZError::CustomMessage(e.to_string()))
    }

    pub fn cleanup(&mut self, zmod: zsys::Module) -> ZResult<i32> {
        let _ = self
            .state
            .stop()
            .map_err(|e| ZError::CustomMessage(e.to_string()))?;

        unsafe {
            Ok(zsys::setfeatureenables(
                zmod,
                &mut self.store.features,
                std::ptr::null_mut(),
            ))
        }
    }

    pub fn trampoline<A>(&mut self, args: &mut A) -> ZResult<A::FeatureOutput>
    where
        A: FeatureDispatch<M>,
    {
        args.get_handler(self.store.registry())
            .execute(self.state.as_mut())
            .map_err(|e| ZError::CustomMessage(e.to_string()))
    }
}
