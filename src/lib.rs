// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.
#![feature(concat_idents)]
#![feature(libc)]
#![allow(improper_ctypes)]

extern crate libc;
extern crate num;

pub use eigenvalues::*;
pub use general_eigenvalues::*;
pub use least_squares::*;
pub use linear_equations::*;

#[macro_use]
mod prefix;
mod scalar;
mod types;
pub mod ll;
pub mod matrix;
pub mod linear_equations;
pub mod least_squares;
pub mod eigenvalues;
pub mod general_eigenvalues;
