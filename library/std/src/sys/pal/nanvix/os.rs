use ::syscall::safe::{FileSystem, FileSystemPath};

use crate::error::Error as StdError;
use crate::ffi::{OsStr, OsString};
use crate::os::nanvix::prelude::*;
use crate::path::{self, PathBuf};
use crate::sys::{error_code_to_error_kind, unsupported};
use crate::{fmt, io, iter, slice};

const PATH_SEPARATOR: u8 = b':';

pub fn errno() -> i32 {
    0
}

pub fn error_string(_errno: i32) -> String {
    "operation successful".to_string()
}

pub fn getcwd() -> io::Result<PathBuf> {
    match FileSystem::get_current_directory() {
        Ok(path) => Ok(PathBuf::from(path.as_str())),
        Err(error) => Err(io::Error::new(error_code_to_error_kind(error.code), error.reason)),
    }
}

pub fn chdir(p: &path::Path) -> io::Result<()> {
    let p = p.to_str().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "path contains invalid UTF-8")
    })?;

    let path = FileSystemPath::new(p)
        .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;

    match FileSystem::change_current_directory(&path) {
        Ok(()) => Ok(()),
        Err(error) => Err(io::Error::new(error_code_to_error_kind(error.code), error.reason)),
    }
}

pub struct SplitPaths<'a> {
    iter: iter::Map<slice::Split<'a, u8, fn(&u8) -> bool>, fn(&'a [u8]) -> PathBuf>,
}

pub fn split_paths(unparsed: &OsStr) -> SplitPaths<'_> {
    fn bytes_to_path(b: &[u8]) -> PathBuf {
        PathBuf::from(<OsStr as OsStrExt>::from_bytes(b))
    }
    fn is_separator(b: &u8) -> bool {
        *b == PATH_SEPARATOR
    }
    let unparsed = unparsed.as_bytes();
    SplitPaths {
        iter: unparsed
            .split(is_separator as fn(&u8) -> bool)
            .map(bytes_to_path as fn(&[u8]) -> PathBuf),
    }
}

impl<'a> Iterator for SplitPaths<'a> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[derive(Debug)]
pub struct JoinPathsError;

pub fn join_paths<I, T>(paths: I) -> Result<OsString, JoinPathsError>
where
    I: Iterator<Item = T>,
    T: AsRef<OsStr>,
{
    let mut joined = Vec::new();

    for (i, path) in paths.enumerate() {
        let path = path.as_ref().as_bytes();
        if i > 0 {
            joined.push(PATH_SEPARATOR)
        }
        if path.contains(&PATH_SEPARATOR) {
            return Err(JoinPathsError);
        }
        joined.extend_from_slice(path);
    }
    Ok(OsStringExt::from_vec(joined))
}

impl fmt::Display for JoinPathsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "path segment contains separator `{}`", char::from(PATH_SEPARATOR))
    }
}

impl StdError for JoinPathsError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        "failed to join paths"
    }
}

pub fn current_exe() -> io::Result<PathBuf> {
    unsupported()
}

pub struct Env(!);

impl Env {
    // FIXME(https://github.com/rust-lang/rust/issues/114583): Remove this when <OsStr as Debug>::fmt matches <str as Debug>::fmt.
    pub fn str_debug(&self) -> impl fmt::Debug + '_ {
        let Self(inner) = self;
        match *inner {}
    }
}

impl fmt::Debug for Env {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(inner) = self;
        match *inner {}
    }
}

impl Iterator for Env {
    type Item = (OsString, OsString);
    fn next(&mut self) -> Option<(OsString, OsString)> {
        let Self(inner) = self;
        match *inner {}
    }
}

pub fn env() -> Env {
    panic!("not supported on this platform")
}

pub fn getenv(_: &OsStr) -> Option<OsString> {
    None
}

pub unsafe fn setenv(_: &OsStr, _: &OsStr) -> io::Result<()> {
    Err(io::const_error!(io::ErrorKind::Unsupported, "cannot set env vars on this platform"))
}

pub unsafe fn unsetenv(_: &OsStr) -> io::Result<()> {
    Err(io::const_error!(io::ErrorKind::Unsupported, "cannot unset env vars on this platform"))
}

pub fn temp_dir() -> PathBuf {
    panic!("no filesystem on this platform")
}

pub fn home_dir() -> Option<PathBuf> {
    None
}

pub fn exit(_code: i32) -> ! {
    crate::intrinsics::abort()
}

pub fn getpid() -> u32 {
    panic!("no pids on this platform")
}
