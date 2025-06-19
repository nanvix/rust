use ::syscall::pthread;
use ::syscall::sysapi::{ffi, sys_types};

pub type Key = sys_types::pthread_key_t;

#[inline]
pub fn create(_dtor: Option<unsafe extern "C" fn(*mut u8)>) -> Key {
    let key = pthread::pthread_key_create().expect("Failed to create pthread key");
    key
}

#[inline]
pub unsafe fn set(key: Key, value: *mut u8) {
    let value = value as *const ffi::c_void;
    let _ = pthread::pthread_setspecific(key, value.into());
}

#[inline]
#[cfg(any(not(target_thread_local), test))]
pub unsafe fn get(key: Key) -> *mut u8 {
    match pthread::pthread_getspecific(key) {
        Ok(value) => <pthread::Pointer as core::convert::Into<*mut u8>>::into(value),
        Err(_error) => crate::ptr::null_mut(),
    }
}

#[inline]
pub unsafe fn destroy(key: Key) {
    pthread::pthread_key_delete(key).expect("Failed to delete pthread key");
}
