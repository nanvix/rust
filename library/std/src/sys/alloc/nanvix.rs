use crate::{
    alloc::{GlobalAlloc, Layout, System},
};

#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: the caller must uphold the safety contract for `malloc`
        unsafe { ::sysalloc::alloc(layout) }

    }

    #[inline]
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // SAFETY: the caller must uphold the safety contract for `malloc`
        unsafe { ::sysalloc::dealloc(_ptr, _layout) }
    }
}
