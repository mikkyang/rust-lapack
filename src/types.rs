// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

#[cfg(feature = "default")]
#[repr(C)]
#[derive(Copy, Clone)]
pub enum Order {
    RowMajor=101,
    ColMajor=102,
}

#[cfg(not(feature = "default"))]
pub enum Order {
    RowMajor,
    ColMajor,
}

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
    pub fn as_i8(&self) -> i8 {
        match self {
            &Transpose::None => 78,
            &Transpose::Conjugate => 67,
        }
    }
}
