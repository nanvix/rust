use ::syscall::sysapi::unistd::{STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO};

use crate::os::fd::{AsFd, AsRawFd};

pub fn is_terminal(fd: &impl AsFd) -> bool {
    let fd = fd.as_fd();
    if fd.as_raw_fd() == STDIN_FILENO
        || fd.as_raw_fd() == STDOUT_FILENO
        || fd.as_raw_fd() == STDERR_FILENO
    {
        true
    } else {
        false
    }
}
