//! # Zsh Module
//! This is a high level crate that allows you to define your own zsh module.
//!
//! ## Getting started
//! To get started, first, you need to create library, not an executable. Then, change your crate
//! type to `"cdylib"` on your `Cargo.toml`:
//! ```toml
//! [lib]
//! crate-type = ["cdylib"]
//! ```
//!
//! ## Boilerplate
//! On your `lib.rs`, you need to put a [`export_module!`] macro call, alongside a `setup` function
//! (can be called whatever you want):
//! ```no_run
//! use zsh_module::{ Module, ModuleBuilder };
//!
//! zsh_module::export_module!(my_module, setup);
//!
//! fn setup() -> Result<Module, Box<dyn std::error::Error>> {
//!    todo!()
//! }
//! ```
//! ## The `setup` function
//! A proper `setup` function must return a [`Result<Module, E>`] where `E` implements
//! [`Error`][std::error::Error]. E.g:
//! ```ignore
//! fn setup() -> Result<Module, Box<dyn std::error::Error>> { .. }
//!
//! fn setup() -> Result<Module, anyhow::Error> { .. }
//!
//! fn setup() -> Result<Module, std::io::Error> { .. }
//! ```
//!
//! ## Storing User Data
//! You can store user data inside a module and have it accessible from any callbacks.
//! Here's an example module, located at  that defines a new `greet` builtin command:
//! ```no_run
//! use zsh_module::{Builtin, MaybeZError, Module, ModuleBuilder, Opts, StringArray};
//!
//! // Notice how this module gets installed as `rgreeter`
//! zsh_module::export_module!(rgreeter, setup);
//!
//! struct Greeter;
//!
//! impl Greeter {
//!     fn greet_cmd(&mut self, _name: &str, _args: StringArray, _opts: Opts) -> MaybeZError {
//!         println!("Hello, world!");
//!         Ok(())
//!     }
//! }
//!
//! fn setup() -> Result<Module, Box<dyn std::error::Error>> {
//!     let module = ModuleBuilder::new(Greeter)
//!         .builtin(Greeter::greet_cmd, Builtin::new("greet"))
//!         .build();
//!     Ok(module)
//! }
//! ```
//!
//! ## Installing
//! When your module is ready, copy your shared library to any folder in your `$module_path`
//! and name it whatever you want, the only requirement is that it ends with your platforms's
//! dynamic loadable library extension.
//!
//! To add a folder to your `$module_path`, add the following code to your `.zshrc`:
//!
//! ```sh no_run
//! typeset -aUg module_path
//! module_path+=($HOME/.zsh/modules)
//! ```
//!
//! For development, you can consider symlinking the library into that folder in your `$module_path`.
//!
//! ```sh no_run
//! ln -s "$PWD/target/debug/libmodule.so" "$HOME/.zsh/modules/module.so"
//! ```
//!
//! If everything went fine, you can load it in zsh using the following command:
//! ```sh no_run
//! zmodload <module-name>
//! ```
//!
//! That is it!

#![allow(deprecated)]

// mod eval;            // TODO: Implement ZSH string evaluation
mod flags;
pub use flags::Flags;
// mod hashtable;
mod lifecycle;
pub use lifecycle::{Activate, Deactivate};
pub mod log;            // TODO: Implement a better logging system that supports conditional compilation and better log levels/formatting
pub mod param;          // TODO: Implement parameter manipulation functions
pub mod types;          // TODO: Revisit the necessity of this module.
pub use crate::types::result::Result;
// pub mod zalloc;      // TODO: Implement GlobalAllocator to use ZSH memory allocation functions https://share.google/aimode/gX8QUewCxER2KXrDg

// TODO: Revisit all of the string modules
pub use crate::types::cstring::ToCString;


mod private;
#[doc(hidden)]
#[deprecated = "the items from the `zsh_module::__` module are an implementation detail; \
    they should not be used directly; if you found a need for this, then you are probably \
    doing something wrong; feel free to open an issue/discussion in our GitHub repository \
    (https://github.com/zsh-rs/zsh-module)"]
pub mod __ {
    pub use crate::private::*;
}

// Re-exported public API
#[doc(inline)]
pub use zsh_module_macros::{state, builtin};
