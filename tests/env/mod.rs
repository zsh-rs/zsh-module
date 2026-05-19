use common::ZshExec;

#[path = "../common/mod.rs"]
pub mod common;

mod array;
mod float;
mod hashmap;
mod integer;
mod string;

#[test]
fn unset() {
    let out = r#"
        unset MY_GET
        getter
    "#
    .zsh_exec()
    .stdout();
    assert!(
        out.contains("Error getting MY_GET"),
        "expected error line for unset MY_GET, got: {out:?}",
    );
}
