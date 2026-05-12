


use crate::types::result::Result;

pub trait Activate: std::panic::UnwindSafe + 'static {
    /// Called after setup and features/enables have completed, to do actual activation:
    /// registering hooks, adding commands, populating tables, etc.
    /// Used for any setup that requires other modules/features already being present."
    fn activate(&mut self) -> Result<()>;
}

pub trait Deactivate: std::panic::UnwindSafe + 'static {
    /// Called when the module is being cleaned up, to do deactivation:
    /// unregistering hooks, removing commands, clearing tables, etc.
    /// Undo what [`activate`] did
    ///
    /// #### This is NOT called when the shell exits
    ///
    fn deactivate(&mut self) -> Result<()>; 
}

pub trait ModuleState: Activate + Deactivate {}
impl<T> ModuleState for T where T: Activate + Deactivate {}