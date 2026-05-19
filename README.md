# `zsh-module`

This crate allows users to define ZSH modules in Rust. It provides a simple API for creating ZSH modules and handling ZSH commands.


## Usage

<!-- TODO: -->


## Roadmap

### ⌛ Core API features
- ✅ Implement basic ZSH builtin functionality
- ➕ Implement support for ZSH conditions
- ➕ Implement support for ZSH math functions
- ➕ Implement support for ZSH parameter hooks
- ⌛ Use proc macros for cleaner syntax
- ➕ Provide examples and documentation

#### Resources:
- [Bash Builtins Crate](https://github.com/ayosec/bash-builtins.rs)


### ➕ Runtime features 
- ➕ Threading
- ➕ FD monitoring
- ➕ Main thread tasks
- ➕ ZLE Rust interface using FFI bindings

> These should all be opt-in features that can be enabled via cargo features for conditional compilation. https://doc.rust-lang.org/cargo/reference/features.html

#### Resources:
- [ChatGPT Convo](https://chatgpt.com/share/69b062e1-a5c4-800a-a4b4-5d560c145212)

### ⌛ Param (`env`) Support
- ✅ Get envs (params) for each type
- ➕ Set envs (params) for each type
- ➕ Implement `From` trait for native types to convert them to `ParamType`

