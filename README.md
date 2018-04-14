# XDG Home Macros

[![Build Status](https://travis-ci.org/quinnjr/xdg-user-macros.svg?branch=master)](https://travis-ci.org/quinnjr/xdg-user-macros)

xdg-home-macros provides macros to aid developers with
properly adhearing to the
[XDG standard](https://wiki.archlinux.org/index.php/XDG_Base_Directory_support)
in user-targeted applications.

This package uses std::env as the primary form
of defining the folder strucutre, but falls back
to the common locations since some
OS implementations do not set the environment
variables and instead rely upon the user to specify
them according to the standard.

Example:
```rust
#[macro_use] extern crate xdg_user_macros;
use std::path::PathBuf;
use std::env::{self, home_dir};
fn main() {
  let path = xdg_data_home!("my-awesome-app");
  let mut expected = home_dir().unwrap();
  expected.push(".local/share/my-awesome-app");
  assert_eq!(path, expected)
}
```

__NOTE__:
The macros provided in this library do __not__ create
the folders associated with the returned PathBuf
from each macro. Folder presence checks _should_ be
handled elsewhere in the application.
