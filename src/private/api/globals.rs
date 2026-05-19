use std::cell::LazyCell;

use crate::private::module::ContainerHooks;

#[linkme::distributed_slice]
pub static BUILTINS: [zsh::builtin];


#[linkme::distributed_slice]
pub static CONDITIONS: [zsh::conddef]; // todo!("Conditions are not yet implemented");

#[linkme::distributed_slice]
pub static MATH_FUNCS: [zsh::mathfunc]; // todo!("Math functions are not yet implemented");

#[linkme::distributed_slice]
pub static PARAMETERS: [zsh::paramdef]; // todo!("Parameter hooks are not yet implemented");


/// To be used if it is found that ZSH is causing segfaults by writing to the non-mutable static
// static mut BUILTINS_MUT: LazyCell<Vec<zsh::builtin>> = LazyCell::new(|| BUILTINS.to_vec());

pub static mut FEATURES: LazyCell<zsh::features> = LazyCell::new(|| zsh::features {
    bn_list: BUILTINS.as_ptr() as _,
    bn_size: BUILTINS.len() as _,
    cd_list: std::ptr::null_mut(),
    cd_size: 0,
    mf_list: std::ptr::null_mut(),
    mf_size: 0,
    pd_list: std::ptr::null_mut(),
    pd_size: 0,
    n_abstract: 0,
});

#[linkme::distributed_slice]
pub static CONTAINERS: [&'static dyn ContainerHooks];