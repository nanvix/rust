//! Unix-specific networking functionality.

#![allow(dead_code)]

#![stable(feature = "unix_socket", since = "1.10.0")]

mod addr;
mod datagram;
mod listener;
mod stream;

#[stable(feature = "unix_socket", since = "1.10.0")]
pub use self::addr::*;
#[stable(feature = "unix_socket", since = "1.10.0")]
pub use self::datagram::*;
#[stable(feature = "unix_socket", since = "1.10.0")]
pub use self::listener::*;
#[stable(feature = "unix_socket", since = "1.10.0")]
pub use self::stream::*;
