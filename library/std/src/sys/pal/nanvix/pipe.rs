use crate::io::{self, BorrowedCursor, IoSlice, IoSliceMut};
use crate::sys_common::{FromInner, IntoInner};
use crate::sys::fd::FileDesc;
use crate::sys::unsupported;
use crate::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};
use crate::fmt;

pub struct AnonPipe(FileDesc);

impl fmt::Debug for AnonPipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AnonPipe").field(&self.0).finish()
    }
}

impl AnonPipe {
    pub fn try_clone(&self) -> io::Result<Self> {
        unsupported()
    }

    pub fn read(&self, _buf: &mut [u8]) -> io::Result<usize> {
        unsupported()
    }

    pub fn read_buf(&self, _buf: BorrowedCursor<'_>) -> io::Result<()> {
        unsupported()
    }

    pub fn read_vectored(&self, _bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        unsupported()
    }

    pub fn is_read_vectored(&self) -> bool {
        false
    }

    pub fn read_to_end(&self, _buf: &mut Vec<u8>) -> io::Result<usize> {
        unsupported()
    }

    pub fn write(&self, _buf: &[u8]) -> io::Result<usize> {
        unsupported()
    }

    pub fn write_vectored(&self, _bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        unsupported()
    }

    pub fn is_write_vectored(&self) -> bool {
        false
    }
}

pub fn read2(_p1: AnonPipe, _v1: &mut Vec<u8>, _p2: AnonPipe, _v2: &mut Vec<u8>) -> io::Result<()> {
        unsupported()
}

impl FromInner<FileDesc> for AnonPipe {
    fn from_inner(fd: FileDesc) -> Self {
        Self(fd)
    }
}

impl IntoInner<FileDesc> for AnonPipe {
    fn into_inner(self) -> FileDesc {
        self.0
    }
}

impl AsRawFd for AnonPipe {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
    self.0.as_raw_fd()
    }
}

impl AsFd for AnonPipe {
    fn as_fd(&self) -> BorrowedFd<'_> {
    self.0.as_fd()
    }
}

impl IntoRawFd for AnonPipe {
    fn into_raw_fd(self) -> RawFd {
    self.0.into_raw_fd()
    }
}

impl FromRawFd for AnonPipe {
    unsafe fn from_raw_fd(_: RawFd) -> Self {
        panic!("creating pipe on this platform is unsupported!")
    }
}

impl FromInner<OwnedFd> for AnonPipe {
    fn from_inner(_: OwnedFd) -> Self {
        panic!("creating pipe on this platform is unsupported!")
    }
}
