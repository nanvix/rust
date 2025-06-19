#![forbid(unsafe_op_in_unsafe_fn)]

cfg_if::cfg_if! {
    if #[cfg(all(unix, not(target_os = "nanvix")))] {
        mod unix;
        pub use unix::{AnonPipe, pipe};
    } else if #[cfg(windows)] {
        mod windows;
        pub use windows::{AnonPipe, pipe};
    } else if #[cfg(target_os = "nanvix")] {
        mod nanvix;
        pub use nanvix::{AnonPipe, pipe};
    } else {
        mod unsupported;
        pub use unsupported::{AnonPipe, pipe};
    }
}
