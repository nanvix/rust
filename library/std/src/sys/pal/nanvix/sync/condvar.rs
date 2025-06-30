use ::syscall::pthread;
use ::syscall::sysapi::{self, errno, sys_types};

use super::Mutex;
use crate::cell::UnsafeCell;
use crate::pin::Pin;
use crate::time::Duration;

pub struct Condvar {
    inner: UnsafeCell<sys_types::pthread_cond_t>,
}

impl Condvar {
    pub fn new() -> Condvar {
        Condvar { inner: UnsafeCell::new(sysapi::pthread::PTHREAD_COND_INITIALIZER) }
    }

    #[inline]
    fn raw(&self) -> *mut sys_types::pthread_cond_t {
        self.inner.get()
    }

    /// # Safety
    /// `init` must have been called on this instance.
    #[inline]
    pub unsafe fn notify_one(self: Pin<&Self>) {
        unsafe { pthread::pthread_cond_signal(&mut *self.raw()).unwrap() };
    }

    /// # Safety
    /// `init` must have been called on this instance.
    #[inline]
    pub unsafe fn notify_all(self: Pin<&Self>) {
        unsafe { pthread::pthread_cond_broadcast(&mut *self.raw()).unwrap() };
    }

    /// # Safety
    /// * `init` must have been called on this instance.
    /// * `mutex` must be locked by the current thread.
    /// * This condition variable may only be used with the same mutex.
    #[inline]
    pub unsafe fn wait(self: Pin<&Self>, mutex: Pin<&Mutex>) {
        unsafe { pthread::pthread_cond_wait(&mut *self.raw(), &mut *mutex.raw()).unwrap() };
    }

    /// # Safety
    /// * `init` must have been called on this instance.
    /// * `mutex` must be locked by the current thread.
    /// * This condition variable may only be used with the same mutex.
    pub unsafe fn wait_timeout(&self, mutex: Pin<&Mutex>, dur: Duration) -> bool {
        let mutex = mutex.raw();

        let timeout = match ::syscall::safe::time::Time::now().unwrap().checked_add_duration(&dur) {
            Some(t) => Some(t.into_system_time()),
            None => None,
        };

        match unsafe { pthread::pthread_cond_timedwait(&mut *self.raw(), &mut *mutex, timeout) } {
            Ok(()) => true,
            Err(e) => {
                if e.code.get() == errno::ETIMEDOUT {
                    false
                } else {
                    panic!("failed to wait on condition variable: {e:?}");
                }
            }
        }
    }
}

// `pthread_condattr_setclock` is unfortunately not supported on these platforms.
impl Condvar {
    pub const PRECISE_TIMEOUT: bool = false;

    /// # Safety
    /// May only be called once per instance of `Self`.
    pub unsafe fn init(self: Pin<&mut Self>) {
        let attr: sys_types::pthread_condattr_t = sys_types::pthread_condattr_t::default();
        unsafe { pthread::pthread_cond_init(&mut *self.raw(), &attr).unwrap() };
    }
}

impl !Unpin for Condvar {}

unsafe impl Sync for Condvar {}
unsafe impl Send for Condvar {}

impl Drop for Condvar {
    #[inline]
    fn drop(&mut self) {
        unsafe { pthread::pthread_cond_destroy(&mut *self.raw()).unwrap() };
    }
}
