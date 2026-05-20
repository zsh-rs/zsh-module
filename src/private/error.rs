// use crate::variable;
use std::{ffi::CString, fmt, path::*};

use crate::private::module::name::module_name;

/// The internal error code type.
pub type ErrorCode = isize;

// TODO: Rewrite all doc comments to use new API stuff
/// A zsh error meant for use in this library internally
///
/// Comes with several useful error types.
#[derive(Debug)]
pub enum ZError {
    Panicked(String),

    HandlerNotFound(String),

    /// A low-level generic return type for zsh internal functions that return integer return types
    ///
    /// TODO: Rewrite zsh-sys stuff to use this (if a better solution cannot be implemented)
    Other(ErrorCode),

    /// An error occurring when evaluating a string
    EvalError(ErrorCode),
    /// An error occurring when sourcing a file
    SourceError(ErrorCode),
    /// The specified file could not be found.
    FileNotFound(PathBuf),

    /// An error defined by the library user.
    CustomMessage(String),

    /// A generic conversion error. The internal String is the error message.
    Conversion(String),
}
impl std::error::Error for ZError {}
impl fmt::Display for ZError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other(i) => write!(f, "Received return code: {i}"),
            Self::HandlerNotFound(h) => write!(f, "No handler found for '{}'", h),
            Self::Panicked(s) => write!(f, "[panic] {s}"),
            Self::EvalError(e) => write!(f, "eval error: {e}"),
            Self::SourceError(e) => write!(f, "source error: {e}"),
            Self::FileNotFound(p) => write!(f, "File not found: {}", p.display()),
            Self::Conversion(msg) => write!(f, "Conversion error: {msg}"),
            Self::CustomMessage(msg) => write!(f, "Error: {msg}"),
        }
    }
}
// impl From<ErrorCode> for ZError {
//     fn from(e: ErrorCode) -> Self {
//         Self::Other(e)
//     }
// }

impl ZError {
    pub(crate) fn exit_code(&self) -> u8 {
        match self {
            Self::Other(_) => 80,
            Self::Panicked(_) => 81,
            Self::EvalError(_) => 82,
            Self::SourceError(_) => 83,
            Self::FileNotFound(_) => 84,
            Self::Conversion(_) => 85,
            Self::HandlerNotFound(_) => 86,
            Self::CustomMessage(_) => 1,
        }
    }
}

// impl<E: std::error::Error + std::fmt::Display> From<E> for ZError {
//     fn from(e: E) -> Self {
//         Self::CustomMessage(e.to_string())
//     }
// }
// impl From<variable::VarError> for ZError {
//     fn from(e: variable::VarError) -> Self {
//         Self::Var(e)
//     }
// }

/// Represents the possibility of a zerror.
/// Only use this for functions that aren't expected to return anything.
pub type MaybeZError = Result<(), ZError>;

/// A [`Result`] wrapper around [`ZError`].
// pub type ZResult<T> = Result<T, ZError>;
pub(crate) enum ZResult<T> {
    Ok(T),
    Err(ZError),
}

impl<T> From<core::result::Result<T, ZError>> for ZResult<T> {
    fn from(res: core::result::Result<T, ZError>) -> Self {
        match res {
            Ok(val) => ZResult::Ok(val),
            Err(e) => ZResult::Err(e),
        }
    }
}

impl ZResult<()> {
    pub fn safe_unwrap(self) -> i32 {
        match self {
            ZResult::Ok(_) => 0,
            ZResult::Err(e) => {
                let err_cstr = CString::new(e.to_string()).unwrap_or_else(|_| {
                    CString::new("An error occurred, but the error message contained a null byte and could not be displayed.").unwrap()
                });

                // TODO: Figure out long term logging strategy
                unsafe { zsh::zerrnam(module_name().as_ptr() as _, err_cstr.as_ptr()) };
                e.exit_code() as i32
            }
        }
    }
}
