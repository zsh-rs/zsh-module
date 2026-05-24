

use super::fixtures::ZshExec;


#[test]
fn get() {
    let out = r#"
        typeset -A MY_HASH=(a 1 b 2 c 3)
        hgetter MY_HASH
    "#
    .zsh_exec()
    .stdout();
    assert_eq!(out.trim(), r#"MY_HASH={"a": "1", "b": "2", "c": "3"}"#);
}

#[test]
fn set() {
    let out = r#"
        hsetter a 1 b 2 c 3
        hgetter MY_HASH
    "#
    .zsh_exec()
    .stdout();
    assert_eq!(out.trim(), r#"hsetter: ok
MY_HASH={"a": "1", "b": "2", "c": "3"}"#);
}
