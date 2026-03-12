use std::fmt::Debug;
use crate::features::context::FeatureContext;
use crate::types::result::Result;

pub trait LoadableModuleState:
    Sized + Default + Debug + std::panic::UnwindSafe + 'static
{
    fn features(&self, _ctx: &mut FeatureContext<Self>); // { unimplemented!("Remove this default implementation before shipping") }

    /// Called after setup and features/enables have completed, to do actual activation:
    /// registering hooks, adding commands, populating tables, etc.
    /// Used for any setup that requires other modules/features already being present."
    fn start(&mut self) -> Result<()> {
        Ok(())
    }

    /// Called when the module is being cleaned up, to do deactivation:
    /// unregistering hooks, removing commands, clearing tables, etc.
    /// Undo what boot_ did, but NOT what setup_ did directly (since that should be cleaned up by finish_).  
    ///
    /// #### This is NOT called when the shell exits
    ///
    fn stop(&mut self) -> Result<()> {
        Ok(())
    }
}
