#![deny(unsafe_op_in_unsafe_fn)]

pub mod common;

cfg_if::cfg_if! {
    if #[cfg(all(target_family = "unix", not(target_os = "nanvix")))] {
        mod unix;
        pub use unix::*;
    } else if #[cfg(target_os = "windows")] {
        mod windows;
        pub use windows::*;
    } else if #[cfg(target_os = "hermit")] {
        mod hermit;
        pub use hermit::*;
    } else if #[cfg(target_os = "solid_asp3")] {
        mod solid;
        pub use solid::*;
    } else if #[cfg(target_os = "uefi")] {
        mod uefi;
        pub use uefi::*;
    } else if #[cfg(target_os = "wasi")] {
        mod wasi;
        pub use wasi::*;
    } else if #[cfg(target_os = "nanvix")] {
        mod nanvix;
        pub use nanvix::*;
    } else {
        mod unsupported;
        pub use unsupported::*;
    }
}
