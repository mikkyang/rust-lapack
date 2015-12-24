// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

#[cfg(not(feature = "default"))]
use libc::c_int;
use types::{
    Symmetry,
};
#[cfg(not(feature = "default"))]
use types::Order;

#[cfg(feature = "default")]
pub use rblas::matrix::{Matrix, BandMatrix};

#[cfg(not(feature = "default"))]
pub trait Matrix<T> {
    fn rows(&self) -> c_int;
    fn cols(&self) -> c_int;
    fn order(&self) -> Order { Order::ColMajor }
    fn as_ptr(&self) -> *const T;
    fn as_mut_ptr(&mut self) -> *mut T;
}

#[cfg(not(feature = "default"))]
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

    pub struct M<T>(pub c_int, pub c_int, pub Vec<T>);

    impl<T> Matrix<T> for M<T> {
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
