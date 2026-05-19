use std::ffi::CString;

use super::ParamError;
use super::helpers::{ValidateIdent, lookup_value};
use super::implementations::{GetEnv, SetEnv};

#[allow(private_bounds)]
pub fn get<T: GetEnv>(name: impl Into<Vec<u8>>) -> Result<T, ParamError> {
    let name = CString::new(name)
        .map_err(|_| ParamError::InvalidIdentifier)?
        .into_bytes_with_nul()
        .validate_ident()?;

    T::get_env(name)
}

#[allow(private_bounds)]
pub fn set<T: SetEnv>(name: impl Into<Vec<u8>>, value: T) -> Result<(), ParamError> {
    let name = CString::new(name)
        .map_err(|_| ParamError::InvalidIdentifier)?
        .into_bytes_with_nul()
        .validate_ident()?;

    if lookup_value(&name, zsh::PM_READONLY).is_ok() {
        return Err(ParamError::ReadOnly);
    }

    value.set_env(name)
}

pub fn unset(name: &str) -> Result<(), ParamError> {
    todo!("Implement unset()")
}
