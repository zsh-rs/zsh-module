

use super::fixtures::ZshExec;



#[test]
fn get() {
    let out = r#"
        MY_ARR=(1 2 3)
        printarr MY_ARR
    "#
    .zsh_exec()
    .stdout();
    assert_eq!(out.trim(), r#"MY_ARR=["1", "2", "3"]"#);
}


#[test]
#[ignore = "not implemented yet"]
fn set() {}


#[test]
fn rejects_non_array() {
    let out = r#"
        NOT_AN_ARRAY="just a string"
        printarr NOT_AN_ARRAY
    "#
    .zsh_exec()
    .stdout();
    assert!(
        out.contains("Error getting env var NOT_AN_ARRAY"),
        "expected error line for type mismatch, got: {out:?}",
    );
}