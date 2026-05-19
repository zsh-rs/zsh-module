use super::common::ZshExec;

#[test]
fn set() {
    let out = r#"
        [[ -z "$MY_SET" ]] || { echo "expected MY_SET to start empty, got: $MY_SET"; exit 1; }
        setter
        echo "$MY_SET"
    "#
    .zsh_exec()
    .stdout();
    assert_eq!(out.trim(), "setter: ok\nhello from rust module");
}

#[test]
fn set_rejects_readonly_env() {
    let out = r#"
        typeset -r MY_SET="locked"
        setter
        echo "after: $MY_SET"
    "#
    .zsh_exec()
    .stdout();
    assert!(
        out.contains("setter: error: ReadOnly"),
        "expected ReadOnly error on readonly param, got: {out:?}",
    );
    assert!(
        out.contains("after: locked"),
        "expected MY_SET to retain its readonly value, got: {out:?}",
    );
}

#[test]
fn get() {
    let out = r#"
        MY_GET="hello from zsh"
        getter
    "#
    .zsh_exec()
    .stdout();
    assert_eq!(out.trim(), r#"MY_GET="hello from zsh""#);
}
