#![doc = include_str!("../README.md")]
#![allow(deprecated)]

// mod eval;            // TODO: Implement ZSH string evaluation
mod flags;
pub use flags::Flags;
// mod hashtable;
mod lifecycle;
pub use lifecycle::{Activate, Deactivate};
// pub mod log;            // TODO: Implement a better logging system that supports conditional compilation and better log levels/formatting
pub mod param;          // TODO: Implement parameter manipulation functions
pub mod types;          // TODO: Revisit the necessity of this module.
pub use crate::types::result::Result;
// pub mod variable;
#[cfg(not(test))]
mod zalloc;

// TODO: Revisit all of the string modules
pub use crate::types::cstring::ToCString;

// Re-exported public API
#[doc(inline)]
pub use zsh_module_macros::{state, builtin, Activate, Deactivate};

#[doc(hidden)]
#[deprecated = "the items from the `zsh_module::__` module are an implementation detail; \
    they should not be used directly; if you found a need for this, then you are probably \
    doing something wrong; feel free to open an issue/discussion in our GitHub repository \
    (https://github.com/zsh-rs/zsh-module)"]
pub mod __ {
    pub use crate::private::*;
}
mod private;