use std::collections::HashMap;
use std::ffi::CStr;

use zsh_module::{builtin, env, flags, state, Activate, Deactivate, Result};

#[state]
#[derive(Debug, Default, Activate, Deactivate)]
pub struct Fixture;

#[builtin("setter")]
fn setter_cmd(_: &mut Fixture, _name: &CStr, _: &[&CStr], _: &flags::Flags) -> Result<()> {
    match env::set("MY_SET", "hello from rust module") {
        Ok(()) => println!("setter: ok"),
        Err(e) => println!("setter: error: {:?}", e),
    }
    Ok(())
}

#[builtin("getter")]
fn getter_cmd(_: &mut Fixture, _name: &CStr, _: &[&CStr], _: &flags::Flags) -> Result<()> {
    match env::get::<String>("MY_GET") {
        Ok(value) => println!("MY_GET={:?}", value),
        Err(e) => println!("Error getting MY_GET: {:?}", e),
    }
    Ok(())
}

#[builtin("igetter")]
fn igetter_cmd(_: &mut Fixture, _: &CStr, _: &[&CStr], _: &flags::Flags) -> Result<()> {
    match env::get::<i64>("MY_INT") {
        Ok(value) => println!("MY_INT={}", value),
        Err(e) => println!("Error getting MY_INT: {:?}", e),
    }
    Ok(())
}

#[builtin("printarr")]
fn printarr_cmd(_: &mut Fixture, _: &CStr, args: &[&CStr], _: &flags::Flags) -> Result<()> {
    args.iter()
        .for_each(|arg| match env::get::<Vec<String>>(arg.to_bytes()) {
            Ok(value) => println!("{}={:?}", arg.to_string_lossy(), value),
            Err(e) => println!("Error getting env var {}: {:?}", arg.to_string_lossy(), e),
        });
    Ok(())
}

#[builtin("isetter")]
fn isetter_cmd(_: &mut Fixture, _: &CStr, args: &[&CStr], _: &flags::Flags) -> Result<()> {
    let value: i64 = args[0].to_string_lossy().parse().unwrap();
    match env::set("MY_INT", value) {
        Ok(()) => println!("isetter: ok"),
        Err(e) => println!("isetter: error: {:?}", e),
    }
    Ok(())
}

#[builtin("asetter")]
fn asetter_cmd(_: &mut Fixture, _: &CStr, args: &[&CStr], _: &flags::Flags) -> Result<()> {
    let arr: Vec<&str> = args.iter().map(|s| s.to_str().unwrap()).collect();
    match env::set("MY_ARR", arr.as_slice()) {
        Ok(()) => println!("asetter: ok"),
        Err(e) => println!("asetter: error: {:?}", e),
    }
    Ok(())
}

#[builtin("hsetter")]
fn hsetter_cmd(_: &mut Fixture, _: &CStr, args: &[&CStr], _: &flags::Flags) -> Result<()> {
    let mut map = HashMap::new();
    for chunk in args.chunks(2) {
        map.insert(
            chunk[0].to_string_lossy().to_string(),
            chunk[1].to_string_lossy().to_string(),
        );
    }
    match env::set("MY_HASH", map) {
        Ok(()) => println!("hsetter: ok"),
        Err(e) => println!("hsetter: error: {:?}", e),
    }
    Ok(())
}

#[builtin("hgetter")]
fn hgetter_cmd(_: &mut Fixture, _: &CStr, args: &[&CStr], _: &flags::Flags) -> Result<()> {
    args.iter()
        .for_each(|arg| match env::get::<HashMap<String, String>>(arg.to_bytes()) {
            Ok(map) => {
                let mut entries: Vec<_> = map.into_iter().collect();
                entries.sort_by(|a, b| a.0.cmp(&b.0));
                let body: Vec<String> = entries
                    .iter()
                    .map(|(k, v)| format!("{k:?}: {v:?}"))
                    .collect();
                println!("{}={{{}}}", arg.to_string_lossy(), body.join(", "));
            }
            Err(e) => println!("Error getting env var {}: {:?}", arg.to_string_lossy(), e),
        });
    Ok(())
}
