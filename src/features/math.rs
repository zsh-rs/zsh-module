use std::ffi::c_char;

use zsh_sys as zsys;


use crate::types::result::Result;


pub type MathHandler<S> = fn(&mut S) -> Result<()>;


type MathArgs = (*mut c_char, i32, *mut zsys::mnumber, i32);
