// Copyright 2017 Lyndon Brown
//
// This file is part of the PulseAudio Rust language linking library.
//
// This library is free software; you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation; either version
// 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this library;
// if not, see <http://www.gnu.org/licenses/>.

//! PulseAudio Rust language linking library.

#![doc(html_logo_url = "https://github.com/jnqnfe/pulse-binding-rust/raw/master/logo.png",
       html_favicon_url = "https://github.com/jnqnfe/pulse-binding-rust/raw/master/favicon.ico")]

#![allow(non_camel_case_types, non_snake_case)]

extern crate libc;

pub use channelmap::*;
pub use context::*;
pub use def::*;
pub use direction::*;
pub use error::*;
pub use format::*;
pub use mainloop::*;
pub use operation::*;
pub use proplist::*;
pub use rtclock::*;
pub use sample::*;
pub use stream::*;
pub use timeval::*;
pub use utf8::*;
pub use util::*;
pub use version::*;
pub use volume::*;
pub use xmalloc::*;

pub mod channelmap;
pub mod context;
pub mod def;
pub mod direction;
pub mod error;
pub mod format;
pub mod mainloop;
pub mod operation;
pub mod proplist;
pub mod rtclock;
pub mod sample;
pub mod stream;
pub mod timeval;
pub mod utf8;
pub mod util;
pub mod version;
pub mod volume;
pub mod xmalloc;

