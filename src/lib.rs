// // Copyright (c) 2018 Joseph R. Quinn <quinn.josephr@protonmail.com>
// SPDX-License-Identifier: ISC

//! xdg-macros provides macros to aid developers with
//! properly adhearing to the
//! [XDG standard](https://wiki.archlinux.org/index.php/XDG_Base_Directory_support)
//! in user-targeted applications.
//!
//! This package uses std::env as the primary form
//! of defining the folder strucutre, but falls back
//! to the common locations since some
//! OS implementations do not set the environment
//! variables and instead rely upon the user to specify
//! them according to the standard.
//!
//! Example:
//! ```
//! # #[macro_use] extern crate xdg_user_macros;
//! # use std::path::PathBuf;
//! # use std::env::{self, home_dir};
//! # fn main() {
//! let path = xdg_data_home!("my-awesome-app");
//! let mut expected = home_dir().unwrap();
//! expected.push(".local/share/my-awesome-app");
//! assert_eq!(path, expected)
//! # }
//! ```
//!
//! __NOTE__:
//! The macros provided in this library do __not__ create
//! the folders associated with the returned PathBuf
//! from each macro. Folder presence checks _should_ be
//! handled elsewhere in the application.

#![allow(unused_macros, unused_imports, dead_code)]

extern crate libc;

#[doc(no_inline)]
use libc::getuid;
#[doc(no_inline)]
use libc::uid_t;
use std::env::{self, home_dir};
use std::path::PathBuf;

/// Returns a PathBuf pointing to what should be defined
/// as the $XDG_CONFIG_HOME environment variable.
#[macro_export]
macro_rules! xdg_config_home {
    ($($x: expr),*) => {{
        let mut path = match env::var_os("XDG_CONFIG_HOME") {
            Some(val) => PathBuf::from(val),
            None => {
                let mut path = home_dir().unwrap();
                path.push(".config");
                path
            }
        };
        $(
            path.push($x);
        )*
        path
    }};
}

/// Returns a PathBuf pointing to what should be defined
/// as the $XDG_CACHE_HOME environment variable.
#[macro_export]
macro_rules! xdg_cache_home {
    ($($x: expr),*) => {{
        let mut path = match env::var_os("XDG_CACHE_HOME") {
            Some(val) => PathBuf::from(val),
            None => {
                let mut path = home_dir().unwrap();
                path.push(".cache");
                path
            }
        };
        $(
            path.push($x);
        )*
        path
    }};
}

/// Returns a PathBuf pointing to what should be defined
/// as the $XDG_DATA_HOME environment variable.
#[macro_export]
macro_rules! xdg_data_home {
    ($($x: expr),*) => {{
        let mut path = match env::var_os("XDG_DATA_HOME") {
            Some(val) => PathBuf::from(val),
            None => {
                let mut path = home_dir().unwrap();
                path.push(".local/share");
                path
            }
        };
        $(
            path.push($x);
        )*
        path
    }};
}

/// Returns a PathBuf pointing to what should be defined
/// as the $XDG_RUNTIME_DIR environment variable.
///
/// This macro preferes to be set by an explicitly defined
/// Environment variable.
#[macro_export]
macro_rules! xdg_runtime_dir{
    ($($x: expr),*) => {{
        let mut path = match env::var_os("XDG_RUNTIME_DIR") {
            Some(val) => PathBuf::from(val),
            None => {
                let run_dir = "/run/user";
                let uid: uid_t = unsafe { getuid() };
                let path = PathBuf::from(format!("{}/{}", run_dir, uid));
                path
            }
        };
        $(
            path.push($x);
        )*
        path
    }};
}

#[cfg(test)]
mod test {
    use super::*;
    use libc::getuid;
    use libc::uid_t;
    use std::env::{self, home_dir};
    use std::path::PathBuf;

    #[test]
    pub fn test_xdg_data_home() {
        let data_home: PathBuf = xdg_data_home!("test");
        let expected = PathBuf::from(format!(
            "{}/{}",
            home_dir().unwrap().to_str().unwrap(),
            ".local/share/test",
        ));
        assert_eq!(expected, data_home)
    }

    #[test]
    pub fn test_xdg_runtime_dir() {
        let runtime_dir: PathBuf = xdg_runtime_dir!();
        let uid: uid_t = unsafe { getuid() };
        let expected = PathBuf::from(format!("{}/{}", "/run/user", uid));
        assert_eq!(expected, runtime_dir)
    }

    #[test]
    pub fn test_xdg_config_dir() {
        let config_home: PathBuf = xdg_config_home!("test");
        let expected = PathBuf::from(format!(
            "{}/{}",
            home_dir().unwrap().to_str().unwrap(),
            ".config/test",
        ));
        assert_eq!(expected, config_home)
    }

    #[test]
    pub fn test_xdg_cache_dir() {
        let config_home: PathBuf = xdg_cache_home!("test");
        let expected = PathBuf::from(format!(
            "{}/{}",
            home_dir().unwrap().to_str().unwrap(),
            ".cache/test",
        ));
        assert_eq!(expected, config_home)
    }

}
