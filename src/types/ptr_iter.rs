use std::ffi::c_char;
use std::ops::Not;

pub(crate) trait PointerIter: Sized {
    fn iter_mut(self) -> impl Iterator<Item = *mut c_char>;
    fn iter(self) -> impl Iterator<Item = *const c_char> {
        self.iter_mut().map(|ptr| ptr as *const c_char)
    }
}

impl PointerIter for *mut *mut c_char {
    fn iter_mut(self) -> impl Iterator<Item = *mut c_char> {
        let mut offset = 0;
        std::iter::from_fn(move || {
            let ptr = unsafe { self.add(offset).read() };
            offset += 1;
            ptr.is_null().not().then_some(ptr)
        })
    }
}
