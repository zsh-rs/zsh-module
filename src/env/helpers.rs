use std::ffi::{CString, OsString, c_char};
use std::os::unix::ffi::OsStringExt;

use super::ParamError;

pub(super) trait Unmetafy {
    fn unmetafy(self) -> Result<String, ParamError>;
}

impl Unmetafy for CString {
    fn unmetafy(self) -> Result<String, ParamError> {
        let mut len = self.count_bytes() as i32;
        let mut data: Vec<u8> = unsafe {
            Vec::from_raw_parts(self.into_raw() as _, len as usize + 1, len as usize + 1)
        };

        unsafe { zsh::unmetafy(data.as_mut_ptr() as _, &mut len) };
        data.truncate(len as usize);

        String::from_utf8(data.clone()).map_err(|e| {
            ParamError::NotUnicode(OsString::from_vec(data), e.utf8_error().valid_up_to())
        })
    }
}

pub(super) trait ValidateIdent {
    /// Validate that the given byte buffer is a valid zsh identifier, returning
    /// an error if not. The buffer must be NUL-terminated and may not contain
    /// interior NULs.
    fn validate_ident(self) -> Result<Self, ParamError>
    where
        Self: Sized;
}

impl ValidateIdent for Vec<u8> {
    fn validate_ident(self) -> Result<Self, ParamError> {
        if unsafe { zsh::isident(self.as_ptr() as *mut c_char) } == 0 {
            return Err(ParamError::InvalidIdentifier);
        }
        Ok(self)
    }
}

pub(super) fn lookup_value(name: &Vec<u8>, mask: u32) -> Result<(), ParamError> {
    // `getvalue` advances `*pptr` past the parsed identifier, so the
    // name pointer must reference a writable buffer that outlives the
    // call. Keep ownership here so it drops at the end of the fn.
    let mut name = name.clone();
    let mut name_ptr = name.as_mut_ptr() as *mut c_char;
    let mut vbuf = zsh::value::default();

    let bracks = mask & (zsh::PM_INTEGER | zsh::PM_EFLOAT | zsh::PM_FFLOAT) != 0;

    let v = unsafe { zsh::getvalue(&mut vbuf, &mut name_ptr, bracks as _).as_mut() }
        .ok_or(ParamError::NotPresent)?;
    let pm = unsafe { v.pm.as_ref() }.ok_or(ParamError::NotPresent)?;

    ((mask & pm.node.flags as u32) == mask)
        .then(|| {})
        .ok_or(ParamError::WrongType(zsh::param_type(pm.node.flags as u32)))
}
