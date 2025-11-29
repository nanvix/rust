// Copyright(c) The Maintainers of Nanvix.
// Licensed under the MIT License.

pub use crate::sys::pal::nanvix::os::{Env, env, getenv, setenv, unsetenv};

pub mod os {
    pub const FAMILY: &str = "";
    pub const OS: &str = "";
    pub const DLL_PREFIX: &str = "";
    pub const DLL_SUFFIX: &str = "";
    pub const DLL_EXTENSION: &str = "";
    pub const EXE_SUFFIX: &str = "";
    pub const EXE_EXTENSION: &str = "";
}
