/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! An Au is an "App Unit" and represents 1/60th of a CSS pixel. It was
//! originally proposed in 2002 as a standard unit of measure in Gecko.
//! See https://bugzilla.mozilla.org/show_bug.cgi?id=177805 for more info.

#![cfg_attr(feature = "plugins", feature(plugin))]
#![cfg_attr(feature = "plugins", feature(custom_derive))]

extern crate heapsize;
extern crate num_traits;
extern crate rustc_serialize;
extern crate serde;

mod app_unit;

pub use app_unit::{Au, MIN_AU, MAX_AU, AU_PER_PX};
