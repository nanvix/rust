use ::syscall::safe::dir::{RawDirectory, RawDirectoryEntry};
use ::syscall::safe::{
    self, FileSystemAttributes, FileSystemPath, FileSystemPermissions, RegularFileOffset,
    RegularFileOpenFlags, RegularFileSeekWhence,
};
use ::syscall::sysapi::sys_stat;

use crate::ffi::{OsStr, OsString};
use crate::fmt;
use crate::hash::{Hash, Hasher};
use crate::io::{self, BorrowedCursor, IoSlice, IoSliceMut, SeekFrom};
use crate::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, RawFd};
use crate::os::nanvix::ffi::OsStrExt;
use crate::path::{Path, PathBuf};
use crate::sys::fd::FileDesc;
pub use crate::sys::fs::common::exists;
use crate::sys::time::SystemTime;
use crate::sys::{error_code_to_error_kind, unsupported};
use crate::sys_common::{AsInner, AsInnerMut, FromInner, IntoInner};

pub struct File(FileDesc);

pub struct FileAttr(FileSystemAttributes);

pub struct ReadDir {
    root: RawDirectory,
}

pub struct DirEntry {
    dirent: RawDirectoryEntry,
}

#[derive(Clone, Debug)]
pub struct OpenOptions {
    read: bool,
    write: bool,
    append: bool,
    truncate: bool,
    create: bool,
    create_new: bool,
    custom_flags: i32,
    mode: u32,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct FileTimes {
    accessed: SystemTime,
    modified: SystemTime,
}

pub struct FilePermissions(safe::FileSystemPermissions);

pub struct FileType(safe::FileType);

#[derive(Debug)]
pub struct DirBuilder {
    mode: u32,
}

impl FileAttr {
    pub fn size(&self) -> u64 {
        let size: i64 = self.0.size().into();
        size as u64
    }

    pub fn perm(&self) -> FilePermissions {
        FilePermissions(self.0.permissions())
    }

    pub fn file_type(&self) -> FileType {
        FileType(self.0.file_type())
    }

    pub fn modified(&self) -> io::Result<SystemTime> {
        match self.0.modified() {
            Ok(time) => Ok(SystemTime::from_time(time)),
            Err(error) => Err(io::Error::new(error_code_to_error_kind(error.code), error.reason)),
        }
    }

    pub fn accessed(&self) -> io::Result<SystemTime> {
        match self.0.accessed() {
            Ok(time) => Ok(SystemTime::from_time(time)),
            Err(error) => Err(io::Error::new(error_code_to_error_kind(error.code), error.reason)),
        }
    }

    pub fn created(&self) -> io::Result<SystemTime> {
        match self.0.created() {
            Ok(time) => Ok(SystemTime::from_time(time)),
            Err(error) => Err(io::Error::new(error_code_to_error_kind(error.code), error.reason)),
        }
    }

    pub fn as_raw_stat(&self) -> &sys_stat::stat {
        self.0.as_raw()
    }
}

impl AsInner<FileSystemAttributes> for FileAttr {
    fn as_inner(&self) -> &FileSystemAttributes {
        &self.0
    }
}

impl Clone for FileAttr {
    fn clone(&self) -> FileAttr {
        FileAttr(self.0)
    }
}

impl FilePermissions {
    pub fn readonly(&self) -> bool {
        self.0.user_can_write() || self.0.group_can_write() || self.0.others_can_write()
    }

    pub fn set_readonly(&mut self, readonly: bool) {
        if readonly {
            self.0 =
                FileSystemPermissions::empty().user_read(true).group_read(true).others_read(true);
        } else {
            self.0 = self.0.user_write(false).group_write(false).others_write(false);
        }
    }

    pub fn mode(&self) -> u32 {
        self.0.into()
    }
}

impl Clone for FilePermissions {
    fn clone(&self) -> FilePermissions {
        FilePermissions(self.0)
    }
}

impl PartialEq for FilePermissions {
    fn eq(&self, other: &FilePermissions) -> bool {
        self.0 == other.0
    }
}

impl Eq for FilePermissions {}

impl fmt::Debug for FilePermissions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl FileTimes {
    pub fn set_accessed(&mut self, _t: SystemTime) {}
    pub fn set_modified(&mut self, _t: SystemTime) {}
}

impl FileType {
    pub fn is_dir(&self) -> bool {
        self.0 == safe::FileType::Directory
    }

    pub fn is_file(&self) -> bool {
        self.0 == safe::FileType::RegularFile
    }

    pub fn is_symlink(&self) -> bool {
        self.0 == safe::FileType::SymbolicLink
    }

    pub fn is_block_device(&self) -> bool {
        self.0 == safe::FileType::BlockDevice
    }

    pub fn is_char_device(&self) -> bool {
        self.0 == safe::FileType::CharacterDevice
    }

    pub fn is_fifo(&self) -> bool {
        self.0 == safe::FileType::Fifo
    }

    pub fn is_socket(&self) -> bool {
        self.0 == safe::FileType::Socket
    }
}

impl Clone for FileType {
    fn clone(&self) -> FileType {
        FileType(self.0)
    }
}

impl Copy for FileType {}

impl PartialEq for FileType {
    fn eq(&self, other: &FileType) -> bool {
        self.0 == other.0
    }
}

impl Eq for FileType {}

impl Hash for FileType {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.0.hash(h);
    }
}

impl fmt::Debug for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl FromInner<u32> for FilePermissions {
    fn from_inner(mode: u32) -> Self {
        FilePermissions(safe::FileSystemPermissions::from(mode))
    }
}

impl fmt::Debug for ReadDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.root, f)
    }
}

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<io::Result<DirEntry>> {
        match safe::dir::readdir(&mut self.root) {
            Ok(Some(raw_entry)) => Some(Ok(DirEntry { dirent: raw_entry })),
            Ok(None) => None,
            Err(error) => {
                Some(Err(io::Error::new(error_code_to_error_kind(error.code), error.reason)))
            }
        }
    }
}

impl DirEntry {
    pub fn path(&self) -> PathBuf {
        PathBuf::from(self.dirent.path().unwrap_or_else(|_error| "".to_string()))
    }

    pub fn file_name(&self) -> OsString {
        let file_name = self.dirent.file_name().unwrap_or_else(|_error| "");
        OsString::from(file_name)
    }

    fn file_name_bytes(&self) -> &[u8] {
        self.dirent.file_name_bytes().unwrap_or_else(|_error| b"")
    }

    pub fn metadata(&self) -> io::Result<FileAttr> {
        let directory_name = FileSystemPath::new(&self.dirent.directory_name())
            .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;
        let file_name = self
            .dirent
            .file_name()
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error.reason))?;
        let file_name = FileSystemPath::new(file_name)
            .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;
        let pathname = directory_name
            .join(&file_name)
            .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;
        let attr = safe::lstat(&pathname)
            .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;
        Ok(FileAttr(attr))
    }

    pub fn file_type(&self) -> io::Result<FileType> {
        let file_type = self.dirent.file_type();
        Ok(FileType(file_type))
    }

    pub fn ino(&self) -> u64 {
        self.dirent.inode_number().into()
    }

    pub fn file_name_os_str(&self) -> &OsStr {
        OsStr::from_bytes(self.file_name_bytes())
    }
}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions {
            read: false,
            write: false,
            append: false,
            truncate: false,
            create: false,
            create_new: false,
            custom_flags: 0,
            mode: 0o666,
        }
    }

    pub fn read(&mut self, read: bool) {
        self.read = read;
    }
    pub fn write(&mut self, write: bool) {
        self.write = write;
    }
    pub fn append(&mut self, append: bool) {
        self.append = append;
    }
    pub fn truncate(&mut self, truncate: bool) {
        self.truncate = truncate;
    }
    pub fn create(&mut self, create: bool) {
        self.create = create;
    }
    pub fn create_new(&mut self, create_new: bool) {
        self.create_new = create_new;
    }

    pub fn custom_flags(&mut self, flags: i32) {
        self.custom_flags = flags;
    }

    pub fn mode(&mut self, mode: u32) {
        self.mode = mode;
    }

    fn into_regular_file_open_flags(&self) -> RegularFileOpenFlags {
        let oflags = if self.read && self.write {
            RegularFileOpenFlags::read_write()
        } else if self.write {
            RegularFileOpenFlags::write_only()
        } else {
            RegularFileOpenFlags::read_only()
        };

        let oflags = if self.append { oflags.set_append(true) } else { oflags };
        let oflags = if self.truncate { oflags.set_truncate(true) } else { oflags };
        let oflags = if self.create { oflags.set_create(true) } else { oflags };
        let oflags =
            if self.create_new { oflags.set_create(true).set_exclusive(true) } else { oflags };

        oflags
    }
}

impl File {
    pub fn open(path: &Path, opts: &OpenOptions) -> io::Result<File> {
        let path = path.to_str().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "path contains invalid UTF-8")
        })?;

        let pathname = FileSystemPath::new(path)
            .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;

        let flags = opts.into_regular_file_open_flags();

        let permissions = Some(
            FileSystemPermissions::empty()
                .user_read(true)
                .user_write(true)
                .group_read(true)
                .group_write(true)
                .others_read(true)
                .others_write(true),
        );

        let rawfd = safe::open(&pathname, &flags, permissions)
            .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;

        Ok(File(unsafe { FileDesc::from_raw_fd(rawfd) }))
    }

    pub fn file_attr(&self) -> io::Result<FileAttr> {
        let rawfd = self.0.as_raw_fd();
        let attr = safe::fstat(rawfd)
            .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;

        Ok(FileAttr(attr))
    }

    pub fn fsync(&self) -> io::Result<()> {
        let rawfd = self.0.as_raw_fd();
        safe::fsync(rawfd)
            .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))
    }

    pub fn datasync(&self) -> io::Result<()> {
        let rawfd = self.0.as_raw_fd();
        safe::fdatasync(rawfd)
            .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))
    }

    pub fn lock(&self) -> io::Result<()> {
        unsupported()
    }

    pub fn lock_shared(&self) -> io::Result<()> {
        unsupported()
    }

    pub fn try_lock(&self) -> io::Result<bool> {
        unsupported()
    }

    pub fn try_lock_shared(&self) -> io::Result<bool> {
        unsupported()
    }

    pub fn unlock(&self) -> io::Result<()> {
        unsupported()
    }

    pub fn truncate(&self, _size: u64) -> io::Result<()> {
        unsupported()
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }

    pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        self.0.read_vectored(bufs)
    }

    #[inline]
    pub fn is_read_vectored(&self) -> bool {
        self.0.is_read_vectored()
    }

    pub fn read_at(&self, _buf: &mut [u8], _offset: u64) -> io::Result<usize> {
        unsupported()
    }

    pub fn read_buf(&self, cursor: BorrowedCursor<'_>) -> io::Result<()> {
        self.0.read_buf(cursor)
    }

    pub fn read_vectored_at(
        &self,
        _bufs: &mut [IoSliceMut<'_>],
        _offset: u64,
    ) -> io::Result<usize> {
        unsupported()
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    pub fn write_vectored(&self, _bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        self.0.write_vectored(_bufs)
    }

    pub fn is_write_vectored(&self) -> bool {
        self.0.is_write_vectored()
    }

    pub fn write_at(&self, _buf: &[u8], _offset: u64) -> io::Result<usize> {
        unsupported()
    }

    pub fn write_vectored_at(&self, _bufs: &[IoSlice<'_>], _offset: u64) -> io::Result<usize> {
        unsupported()
    }

    pub fn flush(&self) -> io::Result<()> {
        Ok(())
    }

    pub fn seek(&self, pos: SeekFrom) -> io::Result<u64> {
        let (whence, pos) = match pos {
            // Casting to `i64` is fine, too large values will end up as
            // negative which will cause an error in `lseek`.
            SeekFrom::Start(off) => {
                (RegularFileSeekWhence::Start, RegularFileOffset::from(off as i64))
            }
            SeekFrom::End(off) => (RegularFileSeekWhence::Current, RegularFileOffset::from(off)),
            SeekFrom::Current(off) => (RegularFileSeekWhence::End, RegularFileOffset::from(off)),
        };

        let rawfd = self.0.as_raw_fd();

        let offset = safe::lseek(rawfd, whence, pos)
            .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;
        let offset: i64 = offset.into();

        Ok(offset as u64)
    }

    pub fn tell(&self) -> io::Result<u64> {
        self.seek(SeekFrom::Current(0))
    }

    pub fn duplicate(&self) -> io::Result<File> {
        self.0.duplicate().map(File)
    }

    pub fn set_permissions(&self, perm: FilePermissions) -> io::Result<()> {
        let rawfd = self.0.as_raw_fd();

        safe::fchmod(rawfd, perm.0)
            .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))
    }

    pub fn set_times(&self, times: FileTimes) -> io::Result<()> {
        let times = [times.accessed.t, times.modified.t];

        safe::futimens(self.0.as_raw_fd(), &times)
            .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))
    }
}

impl DirBuilder {
    pub fn new() -> DirBuilder {
        DirBuilder { mode: 0o777 }
    }

    pub fn mkdir(&self, p: &Path) -> io::Result<()> {
        let path = p.to_str().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "path contains invalid UTF-8")
        })?;

        let pathname = FileSystemPath::new(path)
            .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;

        safe::mkdir(&pathname, FileSystemPermissions::empty())
            .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))
    }

    pub fn set_mode(&mut self, mode: u32) {
        self.mode = mode;
    }
}

impl AsInner<FileDesc> for File {
    fn as_inner(&self) -> &FileDesc {
        &self.0
    }
}

impl AsInnerMut<FileDesc> for File {
    fn as_inner_mut(&mut self) -> &mut FileDesc {
        &mut self.0
    }
}

impl IntoInner<FileDesc> for File {
    fn into_inner(self) -> FileDesc {
        self.0
    }
}

impl FromInner<FileDesc> for File {
    fn from_inner(file_desc: FileDesc) -> Self {
        Self(file_desc)
    }
}

impl AsFd for File {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl AsRawFd for File {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

impl IntoRawFd for File {
    fn into_raw_fd(self) -> RawFd {
        self.0.into_raw_fd()
    }
}

impl FromRawFd for File {
    unsafe fn from_raw_fd(raw_fd: RawFd) -> Self {
        unsafe { Self(FromRawFd::from_raw_fd(raw_fd)) }
    }
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

pub fn readdir(p: &Path) -> io::Result<ReadDir> {
    let path = p.to_str().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "path contains invalid UTF-8")
    })?;

    let pathname = FileSystemPath::new(path)
        .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;

    let raw_dir = safe::dir::opendir(&pathname)
        .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;

    Ok(ReadDir { root: raw_dir })
}

pub fn unlink(p: &Path) -> io::Result<()> {
    let path = p.to_str().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "path contains invalid UTF-8")
    })?;

    let pathname = FileSystemPath::new(path)
        .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;

    safe::unlink(&pathname)
        .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))
}

pub fn rename(old: &Path, new: &Path) -> io::Result<()> {
    let old = old.to_str().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "old path contains invalid UTF-8")
    })?;

    let new = new.to_str().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "new path contains invalid UTF-8")
    })?;

    let old_path = FileSystemPath::new(old)
        .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;

    let new_path = FileSystemPath::new(new)
        .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;

    safe::rename(&old_path, &new_path)
        .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))
}

pub fn set_perm(p: &Path, perm: FilePermissions) -> io::Result<()> {
    let path = p.to_str().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "path contains invalid UTF-8")
    })?;

    let pathname = FileSystemPath::new(path)
        .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;

    safe::chmod(&pathname, perm.0)
        .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))
}

pub fn rmdir(_p: &Path) -> io::Result<()> {
    unsupported()
}

pub fn remove_dir_all(_path: &Path) -> io::Result<()> {
    unsupported()
}

pub fn readlink(p: &Path) -> io::Result<PathBuf> {
    let path = p.to_str().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "path contains invalid UTF-8")
    })?;

    let pathname = FileSystemPath::new(path)
        .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;

    let path = safe::readlink(&pathname)
        .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;

    Ok(PathBuf::from(path.as_str()))
}

pub fn symlink(original: &Path, link: &Path) -> io::Result<()> {
    let original = original.to_str().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "original path contains invalid UTF-8")
    })?;

    let link = link.to_str().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "link path contains invalid UTF-8")
    })?;

    let original_path = FileSystemPath::new(original)
        .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;

    let link_path = FileSystemPath::new(link)
        .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;

    safe::symlink(&original_path, &link_path)
        .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))
}

pub fn link(src: &Path, dst: &Path) -> io::Result<()> {
    let src = src.to_str().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "source path contains invalid UTF-8")
    })?;

    let dst = dst.to_str().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "destination path contains invalid UTF-8")
    })?;

    let src_path = FileSystemPath::new(src)
        .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;

    let dst_path = FileSystemPath::new(dst)
        .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;

    safe::link(&src_path, &dst_path)
        .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))
}

pub fn stat(p: &Path) -> io::Result<FileAttr> {
    let path = p.to_str().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "path contains invalid UTF-8")
    })?;

    let pathname = FileSystemPath::new(path)
        .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;

    let attr = safe::stat(&pathname)
        .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;

    Ok(FileAttr(attr))
}

pub fn lstat(p: &Path) -> io::Result<FileAttr> {
    let path = p.to_str().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "path contains invalid UTF-8")
    })?;

    let pathname = FileSystemPath::new(path)
        .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;

    let attr = safe::lstat(&pathname)
        .map_err(|error| io::Error::new(error_code_to_error_kind(error.code), error.reason))?;

    Ok(FileAttr(attr))
}

pub fn canonicalize(_p: &Path) -> io::Result<PathBuf> {
    unsupported()
}

pub fn copy(_from: &Path, _to: &Path) -> io::Result<u64> {
    unsupported()
}

pub fn chown(_path: &Path, _uid: u32, _gid: u32) -> io::Result<()> {
    unsupported()
}

pub fn fchown(_fd: RawFd, _uid: u32, _gid: u32) -> io::Result<()> {
    unsupported()
}

pub fn lchown(_path: &Path, _uid: u32, _gid: u32) -> io::Result<()> {
    unsupported()
}

pub fn chroot(_dir: &Path) -> io::Result<()> {
    unsupported()
}
