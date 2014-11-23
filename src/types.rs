// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

extern crate libc;

use self::libc::{
    c_double,
    c_float,
    c_int,
    c_void,
};

#[allow(non_camel_case_types)]
pub type CLPK_integer = c_int;
#[allow(non_camel_case_types)]
pub type CLPK_real = c_float;
#[allow(non_camel_case_types)]
pub type CLPK_doublereal = c_double;
#[allow(non_camel_case_types)]
pub type CLPK_complex = c_void;
#[allow(non_camel_case_types)]
pub type CLPK_doublecomplex = c_void;
