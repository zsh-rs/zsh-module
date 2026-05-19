

/// The error type for operations interacting with environment variables.
/// Possibly returned from [`env::get()`] and [`env::set()`].
///
/// [`env::var()`]: var
#[derive(Debug, PartialEq, Eq, Clone)]
/// TODO: maybe use thiserror?
pub enum ParamError {
    /// The specified environment variable was not present in the current
    /// process's environment.
    NotPresent,

    /// The specified environment variable was found, but it did not contain
    /// valid unicode data. The found data is returned as a payload of this
    /// variant.
    NotUnicode(std::ffi::OsString, usize),

    /// Name failed zsh's `isident()` check. Caught in Rust before any
    /// FFI call, so this is the only error guaranteed to be raised
    /// before zsh's `zerr()` writes anything to stderr.
    InvalidIdentifier,

    /// Assignment value was invalid, either contianing null bytes or failing
    /// zsh's `metafy()`check. Caught in Rust before any FFI call, so this is
    /// guaranteed to be raised before zsh's `zerr()` writes anything to stderr.
    InvalidValue,

    /// Existing param has `PM_READONLY` set. Caught in Rust by inspecting
    /// the paramtab entry before calling into `assign*param`.
    ReadOnly,

    /// Existing param has a type that does not match the type of the value
    /// being assigned.
    WrongType(u32),

    /// Zsh rejected the assignment for some other reason (type-of-special-
    /// param mismatch, KSHARRAYS interaction, etc.) and emitted its own
    /// diagnostic via `zerr()`. We can't reliably classify these from C.
    Rejected,
}