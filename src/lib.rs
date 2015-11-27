// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.
#![feature(concat_idents)]
#![feature(libc)]

extern crate libc;
extern crate num;

pub use eigenvalues::ops::*;
pub use general_eigenvalues::ops::*;
pub use least_squares::ops::*;
pub use linear_equations::ops::*;

mod pointer;
#[macro_use]
mod prefix;
mod scalar;
mod types;
pub mod matrix;
pub mod linear_equations;
pub mod least_squares;
pub mod eigenvalues;
pub mod general_eigenvalues;
