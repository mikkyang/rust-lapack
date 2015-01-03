// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.
#![feature(concat_idents)]
#![feature(globs)]
#![feature(macro_rules)]

extern crate libc;
extern crate num;

mod pointer;
#[macro_escape]
mod prefix;
mod scalar;
mod types;
pub mod matrix;
pub mod linear_equations;
