// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use types::{
    CLPK_integer,
    Symmetry,
};

pub trait Matrix<T> {
    fn rows(&self) -> CLPK_integer;
    fn cols(&self) -> CLPK_integer;
    fn as_ptr(&self) -> *const T;
    fn as_mut_ptr(&mut self) -> *mut T;
}

pub trait BandMatrix<T>: Matrix<T> {
    fn sub_diagonals(&self) -> CLPK_integer;
    fn sup_diagonals(&self) -> CLPK_integer;
}

pub trait TridiagonalMatrix<T>: Matrix<T> {
    fn as_ptrs(&self) -> (*const T, *const T, *const T);
    fn as_mut_ptrs(&self) -> (*mut T, *mut T, *mut T);
}

pub trait SymmetricMatrix<T>: Matrix<T> {
    fn symmetry(&self) -> Symmetry;
}

#[cfg(test)]
pub mod tests {
    use matrix::Matrix;
    use types::CLPK_integer;

    impl<T> Matrix<T> for (CLPK_integer, CLPK_integer, Vec<T>) {
        fn rows(&self) -> CLPK_integer {
            let &(rows, _, _) = self;
            rows
        }

        fn cols(&self) -> CLPK_integer {
            let &(_, cols, _) = self;
            cols
        }

        fn as_ptr(&self) -> *const T {
            let &(_, _, ref v) = self;
            v.as_slice().as_ptr()
        }

        fn as_mut_ptr(&mut self) -> *mut T {
            let &(_, _, ref mut v) = self;
            v.as_mut_slice().as_mut_ptr()
        }
    }
}
