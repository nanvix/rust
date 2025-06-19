use ::syscall::error::ErrorCode;
use ::syscall::safe::{StandardError, StandardInput, StandardOutput};

use crate::io::{self};
use crate::sys::error_code_to_error_kind;

pub struct Stdin;
pub struct Stdout;
pub struct Stderr;

impl Stdin {
    pub const fn new() -> Stdin {
        Stdin
    }
}

impl io::Read for Stdin {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        StandardInput::get()
            .read(buf)
            .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))
    }
}

impl Stdout {
    pub const fn new() -> Stdout {
        Stdout
    }
}

impl io::Write for Stdout {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        StandardOutput::get()
            .write(buf)
            .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        StandardOutput::get()
            .synchronize()
            .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))
    }
}

impl Stderr {
    pub const fn new() -> Stderr {
        Stderr
    }
}

impl io::Write for Stderr {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        StandardError::get()
            .write(buf)
            .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))
    }

    // Keep the default write_fmt so the `fmt::Arguments` are still evaluated.

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        StandardError::get()
            .synchronize()
            .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))
    }
}

pub const STDIN_BUF_SIZE: usize = 0;

pub fn is_ebadf(err: &io::Error) -> bool {
    err.raw_os_error() == Some(ErrorCode::BadFile as i32)
}

pub fn panic_output() -> Option<impl io::Write> {
    Some(Stderr::new())
}
