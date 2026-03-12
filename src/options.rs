use std::os::raw::c_char;

pub trait OptHelpers {
    /// Whether the option was set using a minus.
    /// E.g:
    /// ```zsh
    /// command +o # Returns false
    /// command -o # Returns true.
    /// ```
    fn is_minus(&self, c: c_char) -> bool;

    /// Whether the option was set using a plus.
    /// E.g:
    /// ```zsh
    /// command +o # Returns true
    /// command -o # Returns false
    /// ```
    fn is_plus(&self, c: c_char) -> bool;

    /// Whether the option was set.
    /// E.g:
    /// ```zsh
    /// command +o # Returns true
    /// command -o # Returns true
    /// command # Returns false
    /// ```
    fn is_set(&self, c: c_char) -> bool;

    /// Returns the argument passed with the option, if any.
    /// E.g:
    /// ```zsh
    /// command +o example # Returns Some("example")
    /// command -o example2 # Returns Some("example2")
    /// command -o # Returns None
    /// command # Returns None
    /// ```
    fn get_arg(&self, c: c_char) -> Option<&std::ffi::CStr>;
}

// Taken from `zsh.h`
// Let's hope Zsh does not change the implementation of these:
pub type Options = zsh_ffi::options;
impl OptHelpers for Options {
    #[inline(always)]
    fn is_minus(&self, c: c_char) -> bool {
        (1 & self.ind[c as usize]) != 0
    }

    #[inline(always)]
    fn is_plus(&self, c: c_char) -> bool {
        (2 & self.ind[c as usize]) != 0
    }

    #[inline(always)]
    fn is_set(&self, c: c_char) -> bool {
        self.ind[c as usize] != 0
    }

    #[inline(always)]
    fn get_arg<'a>(&'a self, c: c_char) -> Option<&'a std::ffi::CStr> {
        let opt: usize = self.ind[c as usize].into();
        if opt > 3 {
            let p = unsafe { self.args.add((opt >> 2) - 1).read() };
            Some(unsafe { std::ffi::CStr::from_ptr(p) })
        } else {
            None
        }
    }
}
