

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
#[ignore = "not implemented yet"]
fn set() {}
