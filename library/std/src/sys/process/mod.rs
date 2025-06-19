cfg_if::cfg_if! {
    if #[cfg(all(target_family = "unix", not(target_os = "nanvix")))] {
        mod unix;
        use unix as imp;
    } else if #[cfg(target_os = "windows")] {
        mod windows;
        use windows as imp;
    } else if #[cfg(target_os = "uefi")] {
        mod uefi;
        use uefi as imp;
    } else if #[cfg(target_os = "nanvix")] {
        mod nanvix;
        use nanvix as imp;
    } else {
        mod unsupported;
        use unsupported as imp;
    }
}

pub use imp::{
    Command, CommandArgs, EnvKey, ExitCode, ExitStatus, ExitStatusError, Process, Stdio, StdioPipes,
};
