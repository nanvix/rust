#![unstable(reason = "not public", issue = "none", feature = "fd")]

use ::syscall::safe;
use ::syscall::safe::FileDescriptorFlags;

use crate::io::{self, BorrowedCursor, IoSlice, IoSliceMut, Read};
use crate::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};
use crate::sys::error_code_to_error_kind;
use crate::sys_common::{AsInner, FromInner, IntoInner};
use crate::{cmp, slice};

const READ_LIMIT: usize = 4096; // FIXME

#[derive(Debug)]
pub struct FileDesc(OwnedFd);

impl FileDesc {
    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        let rawfd = self.as_raw_fd();

        let n = safe::read(rawfd, buf)
            .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;

        Ok(n)
    }

    pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        io::default_read_vectored(|b| self.read(b), bufs)
    }

    #[inline]
    pub fn is_read_vectored(&self) -> bool {
        false
    }

    pub fn read_to_end(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        let mut me = self;
        (&mut me).read_to_end(buf)
    }

    // TODO: implement read_at().

    pub fn read_buf(&self, mut cursor: BorrowedCursor<'_>) -> io::Result<()> {
        let rawfd = self.as_raw_fd();
        let buf = unsafe {
            slice::from_raw_parts_mut(
                cursor.as_mut().as_mut_ptr(),
                cmp::min(cursor.capacity(), READ_LIMIT),
            )
            .assume_init_mut()
        };

        let n = safe::read(rawfd, buf)
            .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;

        // Safety: `ret` bytes were written to the initialized portion of the buffer
        unsafe {
            cursor.advance_unchecked(n);
        }
        Ok(())
    }

    // TODO: implement read_vectored_at().

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        let rawfd = self.as_raw_fd();

        let n = safe::write(rawfd, buf)
            .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;

        Ok(n)
    }

    pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        io::default_write_vectored(|b| self.write(b), bufs)
    }

    #[inline]
    pub fn is_write_vectored(&self) -> bool {
        false
    }

    // TODO: implement write_at().

    // TODO: implement write_vectored_at().

    pub fn set_cloexec(&self) -> io::Result<()> {
        let fd = self.as_raw_fd();
        let cmd = ::syscall::safe::FileControlRequest::GetFileDescriptorFlags;
        let flags = ::syscall::safe::fcntl(fd, cmd)
            .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;

        let flags = FileDescriptorFlags::from(flags).set_close_on_exec(true);

        let cmd = ::syscall::safe::FileControlRequest::SetFileDescriptorFlags(flags);
        let _ = safe::fcntl(fd, cmd)
            .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason));

        Ok(())
    }

    // TODO: implement set_nonblocking().

    #[inline]
    pub fn duplicate(&self) -> io::Result<FileDesc> {
        Ok(Self(self.0.try_clone()?))
    }
}

impl<'a> Read for &'a FileDesc {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (**self).read(buf)
    }

    fn read_buf(&mut self, cursor: BorrowedCursor<'_>) -> io::Result<()> {
        (**self).read_buf(cursor)
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        (**self).read_vectored(bufs)
    }

    #[inline]
    fn is_read_vectored(&self) -> bool {
        (**self).is_read_vectored()
    }
}

impl AsInner<OwnedFd> for FileDesc {
    #[inline]
    fn as_inner(&self) -> &OwnedFd {
        &self.0
    }
}

impl IntoInner<OwnedFd> for FileDesc {
    fn into_inner(self) -> OwnedFd {
        self.0
    }
}

impl FromInner<OwnedFd> for FileDesc {
    fn from_inner(owned_fd: OwnedFd) -> Self {
        Self(owned_fd)
    }
}

impl AsFd for FileDesc {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl AsRawFd for FileDesc {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

impl IntoRawFd for FileDesc {
    fn into_raw_fd(self) -> RawFd {
        self.0.into_raw_fd()
    }
}

impl FromRawFd for FileDesc {
    unsafe fn from_raw_fd(raw_fd: RawFd) -> Self {
        unsafe { Self(FromRawFd::from_raw_fd(raw_fd)) }
    }
}
