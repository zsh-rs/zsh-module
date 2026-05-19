use std::collections::HashMap;
use std::ffi::{CStr, CString};

use crate::types::string_array::PointerIter;

use super::ParamError;
use super::helpers::{Unmetafy, lookup_value};

pub(super) trait GetEnv: Sized {
    fn get_env(name: Vec<u8>) -> Result<Self, ParamError>;
}

pub(super) trait SetEnv {
    fn set_env(self, name: Vec<u8>) -> Result<(), ParamError>;
}

// ─── Initial impls ─────────────────────────────────────────────────────

impl GetEnv for i64 {
    fn get_env(name: Vec<u8>) -> Result<Self, ParamError> {
        lookup_value(&name, zsh::PM_INTEGER)?;

        unsafe { Ok(zsh::getiparam(name.as_ptr() as _)) }
    }
}

impl GetEnv for f64 {
    /* strict on PM_EFLOAT|PM_FFLOAT, getnparam */
    fn get_env(name: Vec<u8>) -> Result<Self, ParamError> {
        lookup_value(&name, zsh::PM_FFLOAT).or(lookup_value(&name, zsh::PM_EFLOAT))?;

        let value: zsh::mnumber = unsafe { zsh::getnparam(name.as_ptr() as _) };
        todo!("getnparam returns a union of int/float, need to check type and convert to f64")
    }
}

impl GetEnv for String {
    fn get_env(name: Vec<u8>) -> Result<Self, ParamError> {
        lookup_value(&name, zsh::PM_SCALAR)?;

        unsafe { CStr::from_ptr(zsh::getsparam(name.as_ptr() as _)) }
            .to_owned()
            .unmetafy()
    }
}

impl GetEnv for Vec<String> {
    fn get_env(name: Vec<u8>) -> Result<Self, ParamError> {
        lookup_value(&name, zsh::PM_ARRAY)?;

        unsafe { zsh::getaparam(name.as_ptr() as _) }
            .ptr_iter()
            .map(|s| unsafe { CStr::from_ptr(s) }.to_owned().unmetafy())
            .collect()
    }
}

impl GetEnv for HashMap<String, String> {
    fn get_env(name: Vec<u8>) -> Result<Self, ParamError> {
        lookup_value(&name, zsh::PM_HASHED)?;

        let map = unsafe { zsh::gethparam(name.as_ptr() as _) }
            .ptr_iter()
            .map(|s| unsafe { CStr::from_ptr(s) }.to_owned().unmetafy())
            .collect::<Result<Vec<String>, ParamError>>()?
            .chunks_exact(2)
            .map(|pair| (pair[0].clone(), pair[1].clone()))
            .collect();

        Ok(map)
    }
}
// impl FromParam for ParamValue   { /* paramtab → dispatch by PM_TYPE → recurse */ }

impl SetEnv for &str {
    fn set_env(self, mut name: Vec<u8>) -> Result<(), ParamError> {
        let cvalue = CString::new(self).map_err(|_| ParamError::InvalidValue)?;
        let metafied = unsafe { zsh::ztrdup_metafy(cvalue.as_ptr()) };
        if metafied.is_null() {
            return Err(ParamError::InvalidValue);
        }

        unsafe { zsh::setsparam(name.as_mut_ptr() as _, metafied).as_ref() }
            .ok_or(ParamError::Rejected)?;

        Ok(())
    }
}

// impl IntoParam for String       { /* same; takes self to enable steal if cheap */ }
// impl IntoParam for i64          { /* setiparam */ }
// impl IntoParam for f64          { /* setnparam with MN_FLOAT */ }
// impl<S: AsRef<str>> IntoParam for &[S] { /* allocate metafied char**, setaparam */ }
// impl IntoParam for ParamValue   { /* dispatch */ }
