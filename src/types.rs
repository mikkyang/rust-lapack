// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

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
