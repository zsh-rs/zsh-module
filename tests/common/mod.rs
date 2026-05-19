use std::env::consts::{DLL_PREFIX, DLL_SUFFIX};
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::{LazyLock};

const FIXTURE_LIB_NAME: &str = "test_fixture";

static FIXTURE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let fixture_root = PathBuf::from(manifest_dir).join("tests/fixtures/test-fixture");

    let mut cmd = Command::new(env!("CARGO"));
    cmd.arg("build")
        .arg("--manifest-path")
        .arg(fixture_root.join("Cargo.toml"));

    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        cmd.arg("--release");
        "release"
    };

    let status = cmd
        .status()
        .expect("failed to invoke cargo to build test fixture");
    assert!(status.success(), "test fixture cargo build failed");

    let out_dir = fixture_root.join("target").join(profile);

    // zsh resolves `zmodload libtest_fixture` to `<dir>/libtest_fixture<DL_EXT>`,
    // where DL_EXT is `.so` on Linux and on typical macOS zsh builds. Rust's
    // cdylib on macOS emits `.dylib`, so create a sibling `.so` symlink.
    let cargo_output = out_dir.join(format!("{DLL_PREFIX}{FIXTURE_LIB_NAME}{DLL_SUFFIX}"));
    let so_path = out_dir.join(format!("lib{FIXTURE_LIB_NAME}.so"));
    if cargo_output != so_path {
        let _ = std::fs::remove_file(&so_path);
        std::os::unix::fs::symlink(&cargo_output, &so_path)
            .expect("failed to symlink fixture cdylib to .so for zmodload");
    }

    out_dir
});

/// Run a script in zsh with the test fixture cdylib loaded.
///
/// Implemented for `str`, so `String` and `&str` both work via deref.
pub trait ZshExec {
    fn zsh_exec(&self) -> ZshOutput;
}

impl ZshExec for str {
    fn zsh_exec(&self) -> ZshOutput {
        let dir = &FIXTURE_DIR;
        let prelude = format!(
            "module_path+=({}); zmodload lib{FIXTURE_LIB_NAME} || exit 1;",
            dir.display()
        );
        let output = Command::new("zsh")
            .args(["-f", "-c"])
            .arg(format!("{prelude}\n{self}"))
            .output()
            .expect("failed to spawn zsh");
        ZshOutput { inner: output }
    }
}

/// Result of running a zsh script. Holds the raw `std::process::Output`.
pub struct ZshOutput {
    inner: Output,
}

impl ZshOutput {
    /// Asserts the script exited 0 and returns stdout with zsh-module's
    /// `[zsh-module] …` lifecycle log lines stripped so assertions can focus
    /// on builtin output.
    pub fn stdout(self) -> String {
        assert!(
            self.inner.status.success(),
            "zsh exited with {:?}\n--- stderr ---\n{}\n--- stdout ---\n{}",
            self.inner.status.code(),
            String::from_utf8_lossy(&self.inner.stderr),
            String::from_utf8_lossy(&self.inner.stdout),
        );
        let raw = String::from_utf8(self.inner.stdout).expect("zsh stdout was not valid utf-8");
        raw.lines()
            .filter(|line| !line.starts_with("[zsh-module]"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
