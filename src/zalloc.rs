use std::alloc;

struct ZshAllocator;
unsafe impl alloc::GlobalAlloc for ZshAllocator {
    unsafe fn alloc(&self, layout: alloc::Layout) -> *mut u8 {
        unsafe { zsh::zalloc(layout.size()) as _ }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: alloc::Layout) {
        unsafe { zsh::zfree(ptr as _, layout.size().try_into().unwrap()) }
    }

    // unsafe fn realloc(&self, ptr: *mut u8, _: alloc::Layout, new_size: usize) -> *mut u8 {
    //     unsafe { zsh::zrealloc(ptr as _, new_size) as _ }
    // }
}

#[global_allocator]
static Z_ALLOCATOR: ZshAllocator = ZshAllocator;
