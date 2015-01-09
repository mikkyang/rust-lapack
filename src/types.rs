// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.
use libc::{
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

pub enum Symmetry {
    Upper,
    Lower,
}

impl Symmetry {
    pub fn as_i8(self) -> i8 {
        match self {
            Symmetry::Upper => 85,
            Symmetry::Lower => 76,
        }
    }
}

pub enum Transpose {
    None,
    Conjugate,
}

impl Transpose {
    pub fn as_i8(self) -> i8 {
        match self {
            Transpose::None => 78,
            Transpose::Conjugate => 67,
        }
    }
}
