
#[macro_export]
macro_rules! module_hooks {
    ($(fn $name:ident($($arg_name:ident: $arg_type:ty),*) -> $ret:ty;)*) => {$(
        #[unsafe(export_name = concat!(stringify!($name), "_"))]
        extern "C" fn $name(zmod: zsys::Module, $($arg_name: $arg_type),*) -> $ret {
            use_module(|module| module.$name(zmod, $($arg_name),*))
        }
    )*};
}