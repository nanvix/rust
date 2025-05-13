#![deny(unsafe_op_in_unsafe_fn)]

pub mod alloc;

#[path = "../unsupported/os.rs"]
pub mod os;
#[path = "../unsupported/pipe.rs"]
pub mod pipe;
#[path = "../unsupported/time.rs"]
pub mod time;

#[path = "../unsupported/thread.rs"]
pub mod thread;

#[path = "../unsupported/common.rs"]
#[deny(unsafe_op_in_unsafe_fn)]
mod common;
pub use common::*;

use crate::ffi::{c_char, c_int};
use crate::ptr;

unsafe extern "C" {
    fn main(argc: c_int, argv: *const *const c_char) -> c_int;
}

#[unsafe(no_mangle)]
#[allow(unused)]
pub extern "C" fn _start() {
    unsafe {
        main(0, ptr::null());
    };
}

core::arch::global_asm!(
    r#"
    .extern _start

    .globl _do_start

    .section .crt0, "ax"

    _do_start:
        mov ebp, esp
        push ecx
        push edx
        call _start
    1:  jmp 1b
    "#
);
