use std::alloc::{alloc, Layout};
use std::collections::HashMap;
use std::ffi::{c_char, CStr, CString};

use crate::types::ptr_iter::PointerIter;

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

        let _value: zsh::mnumber = unsafe { zsh::getnparam(name.as_ptr() as _) };
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
            .iter()
            .map(|s| unsafe { CStr::from_ptr(s) }.to_owned().unmetafy())
            .collect()
    }
}

impl GetEnv for HashMap<String, String> {
    fn get_env(name: Vec<u8>) -> Result<Self, ParamError> {
        lookup_value(&name, zsh::PM_HASHED)?;

        let keys = unsafe { zsh::gethkparam(name.as_ptr() as _) }
            .iter()
            .map(|s| unsafe { CStr::from_ptr(s) }.to_owned().unmetafy());

        let values = unsafe { zsh::gethparam(name.as_ptr() as _) }
            .iter()
            .map(|s| unsafe { CStr::from_ptr(s) }.to_owned().unmetafy());

        std::iter::zip(keys, values)
            .map(|(k, v)| Ok((k?, v?)))
            .collect::<Result<HashMap<String, String>, ParamError>>()
    }
}

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

impl SetEnv for String {
    /* same; takes self to enable steal if cheap */
    fn set_env(self, _name: Vec<u8>) -> Result<(), ParamError> {
        todo!()
    }
}
impl SetEnv for i64 {
    fn set_env(self, mut name: Vec<u8>) -> Result<(), ParamError> {
        unsafe { zsh::setiparam(name.as_mut_ptr() as _, self as zsh::zlong).as_ref() }
            .ok_or(ParamError::Rejected)?;
        Ok(())
    }
}
impl SetEnv for f64 {
    /* setnparam with MN_FLOAT */
    fn set_env(self, _name: Vec<u8>) -> Result<(), ParamError> {
        todo!()
    }
}

impl<S: AsRef<str>> SetEnv for &[S] {
    fn set_env(self, mut name: Vec<u8>) -> Result<(), ParamError> {
        let count = self.len();
        let size = (count + 1) * std::mem::size_of::<*mut c_char>();
        let arr = unsafe {
            alloc(Layout::from_size_align(size, std::mem::align_of::<*mut c_char>()).unwrap())
                as *mut *mut c_char
        };
        if arr.is_null() {
            return Err(ParamError::Rejected);
        }

        for (i, s) in self.iter().enumerate() {
            let cstr = CString::new(s.as_ref()).map_err(|_| ParamError::InvalidValue)?;
            unsafe {
                arr.add(i).write(zsh::ztrdup_metafy(cstr.as_ptr()));
            }
        }

        unsafe { arr.add(count).write(std::ptr::null_mut()) };

        unsafe { zsh::setaparam(name.as_mut_ptr() as _, arr).as_ref() }
            .ok_or(ParamError::Rejected)?;

        Ok(())
    }
}

impl<S: AsRef<str>> SetEnv for Vec<S> {
    fn set_env(self, name: Vec<u8>) -> Result<(), ParamError> {
        self.as_slice().set_env(name)
    }
}

impl SetEnv for HashMap<String, String> {
    fn set_env(self, mut name: Vec<u8>) -> Result<(), ParamError> {
        let count = self.len();
        let size = (count * 2 + 1) * std::mem::size_of::<*mut c_char>();
        let arr = unsafe {
            alloc(Layout::from_size_align(size, std::mem::align_of::<*mut c_char>()).unwrap())
                as *mut *mut c_char
        };
        if arr.is_null() {
            return Err(ParamError::Rejected);
        }

        for (i, (k, v)) in self.into_iter().enumerate() {
            let key = CString::new(k).map_err(|_| ParamError::InvalidValue)?;
            let val = CString::new(v).map_err(|_| ParamError::InvalidValue)?;

            unsafe {
                arr.add(i * 2).write(zsh::ztrdup_metafy(key.as_ptr()));
                arr.add(i * 2 + 1).write(zsh::ztrdup_metafy(val.as_ptr()));
            }
        }

        unsafe { arr.add(count * 2).write(std::ptr::null_mut()) };

        unsafe { zsh::sethparam(name.as_mut_ptr() as _, arr).as_ref() }
            .ok_or(ParamError::Rejected)?;

        Ok(())
    }
}
