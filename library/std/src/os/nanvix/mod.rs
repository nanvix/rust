#![stable(feature = "raw_ext", since = "1.1.0")]
#![allow(unsafe_op_in_unsafe_fn)]

#[stable(feature = "raw_ext", since = "1.1.0")]
pub use ::sys::*;

pub mod ffi;

/// A prelude for conveniently writing platform-specific code.
///
/// Includes all extension traits, and some important type definitions.
#[stable(feature = "rust1", since = "1.0.0")]
pub mod prelude {
    #[doc(no_inline)]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub use super::ffi::{OsStrExt, OsStringExt};
}
