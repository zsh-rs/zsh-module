

use super::common::ZshExec;


// Surfaces a bug in src/env/implementations.rs: `GetEnv for HashMap<String, String>`
// calls `gethparam(name)` and treats the result as a flat `char**` of alternating
// keys/values, but for associative arrays `gethparam` does not return the table in
// that shape — most pairs are lost. Unignore once the impl uses the correct zsh
// accessor for hashed params.
#[test]
#[ignore = "blocked on HashMap GetEnv impl bug — see test note"]
fn get() {
    let out = r#"
        typeset -A MY_HASH=(a 1 b 2 c 3)
        hgetter MY_HASH
    "#
    .zsh_exec()
    .stdout();
    assert_eq!(out.trim(), r#"MY_HASH={"a": "1", "b": "2", "c": "3"}"#);
}
