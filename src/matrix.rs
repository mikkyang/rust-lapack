// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use types::{
    CLPK_integer,
};

pub trait Matrix<T> {
    fn rows(&self) -> CLPK_integer;
    fn cols(&self) -> CLPK_integer;
    fn as_ptr(&self) -> *const T;
    fn as_mut_ptr(&mut self) -> *mut T;
}
