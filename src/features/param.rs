use std::ffi::c_char;

use zsh_sys as zsys;




type ParamGetArgs = (*mut zsys::param,);
type ParamSetArgs = (*mut zsys::param, *mut c_char);