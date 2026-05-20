use std::ffi::c_char;
use std::ops::Not;

pub(crate) trait PointerIter {
    fn ptr_iter<'a>(self) -> impl Iterator<Item = *const c_char> + 'a;
    fn ptr_iter_mut<'a>(self) -> impl Iterator<Item = *mut c_char> + 'a;
}

impl PointerIter for *mut *mut c_char {
    fn ptr_iter<'a>(self) -> impl Iterator<Item = *const c_char> + 'a {
        self.ptr_iter_mut().map(|ptr| ptr as *const c_char)
    }

    fn ptr_iter_mut<'a>(self) -> impl Iterator<Item = *mut c_char> + 'a {
        let mut offset = 0;
        std::iter::from_fn(move || {
            let ptr = unsafe { self.add(offset).read() };
            offset += 1;
            ptr.is_null().not().then_some(ptr)
        })
    }
}
