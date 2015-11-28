// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use libc::c_int;
use types::{
    Layout,
    Symmetry,
    Transpose,
};

pub trait Matrix<T> {
    fn rows(&self) -> c_int;
    fn cols(&self) -> c_int;
    fn layout(&self) -> Layout { Layout::RowMajor }
    fn transpose(&self) -> Transpose { Transpose::None }
    fn as_ptr(&self) -> *const T;
    fn as_mut_ptr(&mut self) -> *mut T;
}

pub trait BandMatrix<T>: Matrix<T> {
    fn sub_diagonals(&self) -> c_int;
    fn sup_diagonals(&self) -> c_int;
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
    use libc::c_int;
    use matrix::Matrix;

    impl<T> Matrix<T> for (c_int, c_int, Vec<T>) {
        fn rows(&self) -> c_int {
            self.0
        }

        fn cols(&self) -> c_int {
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
