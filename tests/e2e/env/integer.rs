

use super::fixtures::ZshExec;



#[test]
fn get() {
    let out = r#"
        typeset -i MY_INT=42
        igetter
    "#
    .zsh_exec()
    .stdout();
    assert_eq!(out.trim(), "MY_INT=42");
}

#[test]
fn set() {
    let out = r#"
        isetter 42
        igetter
    "#
    .zsh_exec()
    .stdout();
    assert_eq!(out.trim(), "isetter: ok\nMY_INT=42");
}
