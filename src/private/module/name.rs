
pub(crate) fn module_name() -> &'static str {
    option_env!("ZSH_MODULE_NAME").unwrap_or(env!("CARGO_PKG_NAME"))
}

