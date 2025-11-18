#![deny(unsafe_op_in_unsafe_fn)]
#![allow(dead_code)]

pub mod args;
mod common;
pub mod env;
pub mod fd;
pub mod os;
pub mod pipe;
pub mod sync;
pub mod thread;
pub mod time;
use ::syscall::error::ErrorCode;
pub use common::*;

/// Converts an `ErrorCode` from Nanvix to a `std::io::ErrorKind`.
pub fn error_code_to_error_kind(error_code: ErrorCode) -> crate::io::ErrorKind {
    use crate::io::ErrorKind::*;
    match error_code {
        ErrorCode::TooBig => ArgumentListTooLong,
        ErrorCode::AddressInUse => AddrInUse,
        ErrorCode::AddressNotAvailable => AddrNotAvailable,
        ErrorCode::ResourceBusy => ResourceBusy,
        ErrorCode::ConnectionAborted => ConnectionAborted,
        ErrorCode::ConnectionRefused => ConnectionRefused,
        ErrorCode::ConnectionReset => ConnectionReset,
        ErrorCode::DeadlockWouldOccur => Deadlock,
        ErrorCode::QuotaExceeded => QuotaExceeded,
        ErrorCode::EntryExists => AlreadyExists,
        ErrorCode::FileTooLarge => FileTooLarge,
        ErrorCode::HostUnreachable => HostUnreachable,
        ErrorCode::Interrupted => Interrupted,
        ErrorCode::InvalidArgument => InvalidInput,
        ErrorCode::IsDirectory => IsADirectory,
        ErrorCode::SymbolicLinkLoop => FilesystemLoop,
        ErrorCode::NoSuchEntry => NotFound,
        ErrorCode::OutOfMemory => OutOfMemory,
        ErrorCode::NoSpaceOnDevice => StorageFull,
        ErrorCode::InvalidSysCall => Unsupported,
        ErrorCode::TooManyLinks => TooManyLinks,
        ErrorCode::NameTooLong => InvalidFilename,
        ErrorCode::NetworkDown => NetworkDown,
        ErrorCode::NetworkUnreachable => NetworkUnreachable,
        ErrorCode::TransportEndpointNotConnected => NotConnected,
        ErrorCode::InvalidDirectory => NotADirectory,
        ErrorCode::DirectoryNotEmpty => DirectoryNotEmpty,
        ErrorCode::BrokenPipe => BrokenPipe,
        ErrorCode::ReadOnlyFileSystem => ReadOnlyFilesystem,
        ErrorCode::IllegalSeek => NotSeekable,
        ErrorCode::StaleHandle => StaleNetworkFileHandle,
        ErrorCode::OperationTimedOut => TimedOut,
        ErrorCode::TextFileBusy => ExecutableFileBusy,
        ErrorCode::CrossDeviceLink => CrossesDevices,
        ErrorCode::OperationInProgress => InProgress,
        ErrorCode::PermissionDenied | ErrorCode::OperationNotPermitted => PermissionDenied,
        ErrorCode::TryAgain => WouldBlock,
        _ => Uncategorized,
    }
}

pub type RawOsError = ErrorCode;
