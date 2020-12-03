//! `libnotcurses-sys` is an unsafe Rust wrapper for the notcurses C library
//!
//! The API is mostly the original one, while following the Rust API Guidelines.
//!
//! For a safer wrapper with a more idiomatic API for Rust, you can use
//! [](https://crates.io/crates/notcurses)
//! [notcurses-rs](https://github.com/dankamongmen/notcurses-rs)
//!
//! ### notcurses C API docs:
//!
//! - [Doxygen Documentation](https://nick-black.com/notcurses/html/index.html)
//! - [API reference (man pages)](https://nick-black.com/notcurses/)
//! - [Wiki](https://nick-black.com/dankwiki/index.php/Notcurses)
//! - [The Book Guide (pdf)](https://nick-black.com/htp-notcurses.pdf)
//! - [USAGE.md](https://github.com/dankamongmen/notcurses/blob/master/USAGE.md)
//!
#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]
#![allow(clippy::too_many_arguments)]


mod bindings;
#[doc(inline)]
pub use bindings::*;

mod cells;
mod channel;
mod direct;
mod error;
mod file;
mod input;
mod key;
mod keycodes;
mod macros;
mod metric;
mod notcurses;
mod palette;
mod pixel;
mod plane;
mod time;
mod visual;
mod widgets;

pub use cells::*;
pub use channel::*;
pub use direct::*;
pub use error::*;
pub use file::*;
pub use input::*;
pub use key::*;
pub use keycodes::*;
pub use macros::*;
pub use metric::*;
pub use notcurses::*;
pub use palette::*;
pub use pixel::*;
pub use plane::*;
pub use time::*;
pub use visual::*;
pub use widgets::*;
