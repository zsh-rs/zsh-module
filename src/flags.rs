use std::ffi::{CStr, c_char};

#[repr(transparent)]
pub struct Flags<'a>(&'a zsh::options);

impl<'a> From<*mut zsh::options> for Flags<'a> {
    fn from(value: *mut zsh::options) -> Self {
        Self(unsafe { &*value })
    }
}

// Taken from `zsh.h`
// Let's hope Zsh does not change the implementation of these:
impl<'a> Flags<'a> {
    /// Whether the option was set using a minus.
    /// E.g:
    /// ```zsh
    /// command +o # Returns false
    /// command -o # Returns true.
    /// ```
    #[inline(always)]
    pub fn is_minus(&self, c: c_char) -> bool {
        (1 & self.0.ind[c as usize]) != 0
    }

    /// Whether the option was set using a plus.
    /// E.g:
    /// ```zsh
    /// command +o # Returns true
    /// command -o # Returns false
    /// ```
    #[inline(always)]
    pub fn is_plus(&self, c: c_char) -> bool {
        (2 & self.0.ind[c as usize]) != 0
    }

    /// Whether the option was set.
    /// E.g:
    /// ```zsh
    /// command +o # Returns true
    /// command -o # Returns true
    /// command # Returns false
    /// ```
    #[inline(always)]
    pub fn is_set(&self, c: c_char) -> bool {
        self.0.ind[c as usize] != 0
    }

    /// Returns the argument passed with the option, if any.
    /// E.g:
    /// ```zsh
    /// command +o example # Returns Some("example")
    /// command -o example2 # Returns Some("example2")
    /// command -o # Returns None
    /// command # Returns None
    /// ```
    #[inline(always)]
    pub fn get_arg(&self, c: c_char) -> Option<&CStr> {
        let opt: usize = self.0.ind[c as usize].into();
        if opt > 3 {
            let p = unsafe { self.0.args.add((opt >> 2) - 1).read() };
            Some(unsafe { CStr::from_ptr(p) })
        } else {
            None
        }
    }
}
