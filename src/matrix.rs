// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use types::{
    CLPK_integer,
    Symmetry,
    Transpose,
};

pub trait Matrix<T> {
    fn rows(&self) -> CLPK_integer;
    fn cols(&self) -> CLPK_integer;
    fn transpose(&self) -> Transpose { Transpose::None }
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
            self.0
        }

        fn cols(&self) -> CLPK_integer {
            self.1
        }

        fn as_ptr(&self) -> *const T {
            self.2[..].as_ptr()
        }

        fn as_mut_ptr(&mut self) -> *mut T {
            (&mut self.2[..]).as_mut_ptr()
        }
    }
}
