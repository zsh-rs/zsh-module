# `zsh-module` [![Crates.io](https://img.shields.io/crates/v/zsh-module.svg)](https://crates.io/crates/zsh-module) [![Documentation](https://docs.rs/zsh-module/badge.svg)](https://docs.rs/zsh-module) [![License](https://img.shields.io/crates/l/zsh-module.svg)](https://crates.io/crates/zsh-module)

This crate allows users to define ZSH modules in Rust. It provides a simple API for creating ZSH modules and handling ZSH commands.



This is a high level crate that allows you to define your own zsh module.

## Getting started
To get started, first, you need to create library, not an executable. Then, change your crate
type to `"cdylib"` on your `Cargo.toml`:
```toml
[lib]
crate-type = ["cdylib"]
```

## Boilerplate
On your `lib.rs`, you need to put a `export_module!` macro call, alongside a `setup` function
(can be called whatever you want):
```rust
use zsh_module::{ Module, ModuleBuilder };

zsh_module::export_module!(my_module, setup);

fn setup() -> Result<Module, Box<dyn std::error::Error>> {
   todo!()
}
```
## The `setup` function
A proper `setup` function must return a `Result<Module, E>` where `E` implements
`std::error::Error`. E.g:
```rust
fn setup() -> Result<Module, Box<dyn std::error::Error>> { .. }

fn setup() -> Result<Module, anyhow::Error> { .. }

fn setup() -> Result<Module, std::io::Error> { .. }
```

## Storing User Data
You can store user data inside a module and have it accessible from any callbacks.
Here's an example module, located at  that defines a new `greet` builtin command:
```rust
use zsh_module::{Builtin, MaybeZError, Module, ModuleBuilder, Opts, StringArray};

// Notice how this module gets installed as `rgreeter`
zsh_module::export_module!(rgreeter, setup);

struct Greeter;

impl Greeter {
    fn greet_cmd(&mut self, _name: &str, _args: StringArray, _opts: Opts) -> MaybeZError {
        println!("Hello, world!");
        Ok(())
    }
}

fn setup() -> Result<Module, Box<dyn std::error::Error>> {
    let module = ModuleBuilder::new(Greeter)
        .builtin(Greeter::greet_cmd, Builtin::new("greet"))
        .build();
    Ok(module)
}
```

## Installing
When your module is ready, copy your shared library to any folder in your `$module_path`
and name it whatever you want, the only requirement is that it ends with your platforms's
dynamic loadable library extension.

To add a folder to your `$module_path`, add the following code to your `.zshrc`:

```sh no_run
typeset -aUg module_path
module_path+=($HOME/.zsh/modules)
```

For development, you can consider symlinking the library into that folder in your `$module_path`.

```sh no_run
ln -s "$PWD/target/debug/libmodule.so" "$HOME/.zsh/modules/module.so"
```

If everything went fine, you can load it in zsh using the following command:
```sh no_run
zmodload <module-name>
```

That is it!