use super::{CONTAINERS, FEATURES};
use crate::private::module::name::module_name;
use std::ffi::{c_char, c_int};

#[unsafe(no_mangle)]
pub extern "C" fn setup_(_: zsh::Module) -> c_int {
    println!("[{}] setup_", module_name());
    CONTAINERS
        .iter()
        .for_each(|container| container.init());
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn features_(zmod: zsh::Module, features_ptr: *mut *mut *mut c_char) -> c_int {
    println!("[{}] features_", module_name());
    unsafe { *features_ptr = zsh::featuresarray(zmod, &raw mut *FEATURES) }
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn enables_(zmod: zsh::Module, enables_ptr: *mut *mut c_int) -> c_int {
    println!("[{}] enables_", module_name());
    unsafe { zsh::handlefeatures(zmod, &raw mut *FEATURES, enables_ptr) }
}

#[unsafe(no_mangle)]
pub extern "C" fn boot_(_zmod: zsh::Module) -> c_int {
    println!("[{}] boot_", module_name());
    CONTAINERS
        .iter()
        .map(|c| c.with_state(Box::new(|state| state.activate())).unwrap_or(65))
        .find(|&code| code != 0)
        .unwrap_or(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn cleanup_(zmod: zsh::Module) -> c_int {
    println!("[{}] cleanup_", module_name());
    let exit_code = CONTAINERS
        .iter()
        .map(|c| c.with_state(Box::new(|state| state.deactivate())).unwrap_or(65))
        .find(|&code| code != 0)
        .unwrap_or(0);
    if exit_code != 0 {
        return exit_code;
    }
    unsafe { zsh::setfeatureenables(zmod, &raw mut *FEATURES, std::ptr::null_mut()) }
}

#[unsafe(no_mangle)]
pub extern "C" fn finish_(_: zsh::Module) -> c_int {
    println!("[{}] finish_", module_name());
    CONTAINERS
        .iter()
        .for_each(|container| container.drop_data());
    0
}
