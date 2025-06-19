#[stable(feature = "rust1", since = "1.0.0")]
pub use ::syscall::sysapi::{sys_socket::socklen_t, sys_un::sockaddr_un};

use crate::path::Path;
use crate::sealed::Sealed;
use crate::sys::unsupported;
use crate::{fmt, io};

pub(super) fn sockaddr_un(_path: &Path) -> io::Result<(sockaddr_un, socklen_t)> {
    unsupported()
}

enum AddressKind {
    Unnamed,
}

/// An address associated with a Unix socket.
///
/// # Examples
///
/// ```
/// use std::os::unix::net::UnixListener;
///
/// let socket = match UnixListener::bind("/tmp/sock") {
///     Ok(sock) => sock,
///     Err(e) => {
///         println!("Couldn't bind: {e:?}");
///         return
///     }
/// };
/// let addr = socket.local_addr().expect("Couldn't get local address");
/// ```
#[derive(Clone)]
#[stable(feature = "unix_socket", since = "1.10.0")]
pub struct SocketAddr {
    pub(super) addr: sockaddr_un,
    pub(super) len: socklen_t,
}

impl SocketAddr {
    /// Constructs a `SockAddr` with the family `AF_UNIX` and the provided path.
    ///
    /// # Errors
    ///
    /// Returns an error if the path is longer than `SUN_LEN` or if it contains
    /// NULL bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::os::unix::net::SocketAddr;
    /// use std::path::Path;
    ///
    /// # fn main() -> std::io::Result<()> {
    /// let address = SocketAddr::from_pathname("/path/to/socket")?;
    /// assert_eq!(address.as_pathname(), Some(Path::new("/path/to/socket")));
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Creating a `SocketAddr` with a NULL byte results in an error.
    ///
    /// ```
    /// use std::os::unix::net::SocketAddr;
    ///
    /// assert!(SocketAddr::from_pathname("/path/with/\0/bytes").is_err());
    /// ```
    #[stable(feature = "unix_socket_creation", since = "1.61.0")]
    pub fn from_pathname<P>(path: P) -> io::Result<SocketAddr>
    where
        P: AsRef<Path>,
    {
        sockaddr_un(path.as_ref()).map(|(addr, len)| SocketAddr { addr, len })
    }

    /// Returns `true` if the address is unnamed.
    ///
    /// # Examples
    ///
    /// A named address:
    ///
    /// ```no_run
    /// use std::os::unix::net::UnixListener;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let socket = UnixListener::bind("/tmp/sock")?;
    ///     let addr = socket.local_addr().expect("Couldn't get local address");
    ///     assert_eq!(addr.is_unnamed(), false);
    ///     Ok(())
    /// }
    /// ```
    ///
    /// An unnamed address:
    ///
    /// ```
    /// use std::os::unix::net::UnixDatagram;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let socket = UnixDatagram::unbound()?;
    ///     let addr = socket.local_addr().expect("Couldn't get local address");
    ///     assert_eq!(addr.is_unnamed(), true);
    ///     Ok(())
    /// }
    /// ```
    #[must_use]
    #[stable(feature = "unix_socket", since = "1.10.0")]
    pub fn is_unnamed(&self) -> bool {
        matches!(self.address(), AddressKind::Unnamed)
    }

    /// Returns the contents of this address if it is a `pathname` address.
    ///
    /// # Examples
    ///
    /// With a pathname:
    ///
    /// ```no_run
    /// use std::os::unix::net::UnixListener;
    /// use std::path::Path;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let socket = UnixListener::bind("/tmp/sock")?;
    ///     let addr = socket.local_addr().expect("Couldn't get local address");
    ///     assert_eq!(addr.as_pathname(), Some(Path::new("/tmp/sock")));
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Without a pathname:
    ///
    /// ```
    /// use std::os::unix::net::UnixDatagram;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let socket = UnixDatagram::unbound()?;
    ///     let addr = socket.local_addr().expect("Couldn't get local address");
    ///     assert_eq!(addr.as_pathname(), None);
    ///     Ok(())
    /// }
    /// ```
    #[stable(feature = "unix_socket", since = "1.10.0")]
    #[must_use]
    pub fn as_pathname(&self) -> Option<&Path> {
        None
    }

    fn address(&self) -> AddressKind {
        AddressKind::Unnamed
    }
}

#[stable(feature = "unix_socket_abstract", since = "1.70.0")]
impl Sealed for SocketAddr {}

#[stable(feature = "unix_socket", since = "1.10.0")]
impl fmt::Debug for SocketAddr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.address() {
            AddressKind::Unnamed => write!(fmt, "(unnamed)"),
        }
    }
}
