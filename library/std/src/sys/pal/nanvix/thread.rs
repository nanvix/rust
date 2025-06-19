#![allow(fuzzy_provenance_casts)]

use ::syscall::sysapi::ffi::c_void;
use ::syscall::sysapi::sys_types::pthread_t;

use crate::ffi::CStr;
use crate::io;
use crate::num::NonZero;
use crate::sys::error_code_to_error_kind;
use crate::time::Duration;

pub struct Thread {
    id: pthread_t,
}

// Some platforms may have pthread_t as a pointer in which case we still want
// a thread to be Send/Sync
unsafe impl Send for Thread {}
unsafe impl Sync for Thread {}
pub const DEFAULT_MIN_STACK_SIZE: usize = 64 * 1024;

impl Thread {
    // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    pub unsafe fn new(_stack: usize, p: Box<dyn FnOnce()>) -> io::Result<Thread> {
        ::syscall::syslog::trace!("Thread::new()");
        let p = Box::into_raw(Box::new(p));

        let native: pthread_t =
            ::syscall::pthread::pthread_create(thread_start, p as *mut _ as usize).map_err(
                |error| {
                    unsafe { drop(Box::from_raw(p)) };
                    io::Error::new(error_code_to_error_kind(error.code), error.reason)
                },
            )?;

        return Ok(Thread { id: native });

        extern "C" fn thread_start(arg: usize) -> usize {
            let main: *mut c_void = arg as *mut c_void;
            unsafe {
                // Finally, let's run some code.
                Box::from_raw(main as *mut Box<dyn FnOnce()>)();
            }
            0
        }
    }

    pub fn yield_now() {
        ::syscall::syslog::trace!("Thread::yield_now()");
        // do nothing
    }

    pub fn set_name(_name: &CStr) {
        ::syscall::syslog::trace!("Thread::set_name()");
        // nope
    }

    pub fn sleep(_dur: Duration) {
        ::syscall::syslog::trace!("Thread::sleep()");
    }

    pub fn join(self) {
        ::syscall::syslog::trace!("Thread::join()");
        let id = self.id;
        ::syscall::pthread::pthread_join(id)
            .map_err(|error| {
                assert!(
                    false,
                    "failed to join thread: {}",
                    io::Error::new(error_code_to_error_kind(error.code), error.reason)
                );
            })
            .unwrap();
    }
}

pub fn available_parallelism() -> io::Result<NonZero<usize>> {
    ::syscall::syslog::trace!("Thread::available_parallelism()");
    Ok(NonZero::new(1).expect("available_parallelism should never return zero"))
}
