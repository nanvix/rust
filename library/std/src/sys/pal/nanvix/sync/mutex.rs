use ::syscall::error::Error;
use ::syscall::pthread;
use ::syscall::sysapi::{self, sys_types};

use crate::cell::UnsafeCell;
use crate::pin::Pin;

pub struct Mutex {
    inner: UnsafeCell<sys_types::pthread_mutex_t>,
}

impl Mutex {
    pub fn new() -> Mutex {
        Mutex { inner: UnsafeCell::new(sysapi::pthread::PTHREAD_MUTEX_INITIALIZER) }
    }

    pub(super) fn raw(&self) -> *mut sys_types::pthread_mutex_t {
        self.inner.get()
    }

    /// # Safety
    /// May only be called once per instance of `Self`.
    pub unsafe fn init(self: Pin<&mut Self>) {
        // Issue #33770
        //
        // A pthread mutex initialized with PTHREAD_MUTEX_INITIALIZER will have
        // a type of PTHREAD_MUTEX_DEFAULT, which has undefined behavior if you
        // try to re-lock it from the same thread when you already hold a lock
        // (https://pubs.opengroup.org/onlinepubs/9699919799/functions/pthread_mutex_init.html).
        // This is the case even if PTHREAD_MUTEX_DEFAULT == PTHREAD_MUTEX_NORMAL
        // (https://github.com/rust-lang/rust/issues/33770#issuecomment-220847521) -- in that
        // case, `pthread_mutexattr_settype(PTHREAD_MUTEX_DEFAULT)` will of course be the same
        // as setting it to `PTHREAD_MUTEX_NORMAL`, but not setting any mode will result in
        // a Mutex where re-locking is UB.
        //
        // In practice, glibc takes advantage of this undefined behavior to
        // implement hardware lock elision, which uses hardware transactional
        // memory to avoid acquiring the lock. While a transaction is in
        // progress, the lock appears to be unlocked. This isn't a problem for
        // other threads since the transactional memory will abort if a conflict
        // is detected, however no abort is generated when re-locking from the
        // same thread.
        //
        // Since locking the same mutex twice will result in two aliasing &mut
        // references, we instead create the mutex with type
        // PTHREAD_MUTEX_NORMAL which is guaranteed to deadlock if we try to
        // re-lock it from the same thread, thus avoiding undefined behavior.
        let attr: sys_types::pthread_mutexattr_t = sys_types::pthread_mutexattr_t::default();
        unsafe { pthread::pthread_mutex_init(&mut *self.raw(), &attr).unwrap() };
    }

    /// # Safety
    /// * If `init` was not called on this instance, reentrant locking causes
    ///   undefined behaviour.
    /// * Destroying a locked mutex causes undefined behaviour.
    pub unsafe fn lock(self: Pin<&Self>) {
        #[cold]
        #[inline(never)]
        fn fail(error: Error) -> ! {
            panic!("failed to lock mutex: {error:?}");
        }

        let r = unsafe { pthread::pthread_mutex_lock(&mut *self.raw()) };
        // As we set the mutex type to `PTHREAD_MUTEX_NORMAL` above, we expect
        // the lock call to never fail. Unfortunately however, some platforms
        // (Solaris) do not conform to the standard, and instead always provide
        // deadlock detection. How kind of them! Unfortunately that means that
        // we need to check the error code here. To save us from UB on other
        // less well-behaved platforms in the future, we do it even on "good"
        // platforms like macOS. See #120147 for more context.
        if let Err(r) = r {
            fail(r)
        }
    }

    /// # Safety
    /// * If `init` was not called on this instance, reentrant locking causes
    ///   undefined behaviour.
    /// * Destroying a locked mutex causes undefined behaviour.
    pub unsafe fn try_lock(self: Pin<&Self>) -> bool {
        unsafe { pthread::pthread_mutex_trylock(&mut *self.raw()).is_ok() }
    }

    /// # Safety
    /// The mutex must be locked by the current thread.
    pub unsafe fn unlock(self: Pin<&Self>) {
        let r = unsafe { pthread::pthread_mutex_unlock(&mut *self.raw()) };
        debug_assert_eq!(r.is_ok(), true);
    }
}

impl !Unpin for Mutex {}

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {}

impl Drop for Mutex {
    fn drop(&mut self) {
        // SAFETY:
        // If `lock` or `init` was called, the mutex must have been pinned, so
        // it is still at the same location. Otherwise, `inner` must contain
        // `PTHREAD_MUTEX_INITIALIZER`, which is valid at all locations. Thus,
        // this call always destroys a valid mutex.
        let r = unsafe { pthread::pthread_mutex_destroy(&mut *self.raw()) };
        debug_assert_eq!(r.is_ok(), true);
    }
}
