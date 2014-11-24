// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

extern crate num;

use self::num::complex::{
    Complex32,
    Complex64,
};
use linear_equations::ll;
use matrix::{
    Matrix,
    BandMatrix,
    SymmetricMatrix,
    TridiagonalMatrix,
};
use pointer::CPtr;
use scalar::Scalar;
use types::{
    CLPK_integer,
    Symmetry,
};

pub trait Gesv {
    fn gesv(a: &mut Matrix<Self>, b: &mut Matrix<Self>, p: &mut Matrix<CLPK_integer>);
}

macro_rules! gesv_impl(
    ($t: ty, $ll: ident) => (
        impl Gesv for $t {
            fn gesv(a: &mut Matrix<$t>, b: &mut Matrix<$t>, p: &mut Matrix<CLPK_integer>) {
                unsafe {
                    let mut info: CLPK_integer = 0;

                    ll::$ll(a.cols().as_const(), b.cols().as_const(),
                        a.as_mut_ptr().as_c_ptr(), a.rows().as_const(),
                        p.as_mut_ptr().as_c_ptr(),
                        b.as_mut_ptr().as_c_ptr(), b.rows().as_const(),
                        &mut info as *mut CLPK_integer);
                }
            }
        }
    );
)

gesv_impl!(f32,         sgesv_)
gesv_impl!(f64,         dgesv_)
gesv_impl!(Complex32,   cgesv_)
gesv_impl!(Complex64,   zgesv_)

#[cfg(test)]
mod gesv_tests {

    use linear_equations::ops::Gesv;

    #[test]
    fn real() {
        let mut a = (2i32, 2i32, vec![1.0f64, 4.0, 1.0, 2.0]);
        let mut b = (2i32, 1i32, vec![-2.0f64, 2.0]);
        let mut p = (2i32, 2i32, vec![0i32, 0]);

        Gesv::gesv(&mut a, &mut b, &mut p);

        let (_, _, x) = b;
        assert_eq!(x, vec![3.0f64, -5.0]);
    }
}

pub trait Gbsv {
    fn gbsv(a: &mut BandMatrix<Self>, b: &mut Matrix<Self>, p: &mut Matrix<CLPK_integer>);
}

macro_rules! gbsv_impl(
    ($t: ty, $ll: ident) => (
        impl Gbsv for $t {
            fn gbsv(a: &mut BandMatrix<$t>, b: &mut Matrix<$t>, p: &mut Matrix<CLPK_integer>) {
                unsafe {
                    let mut info: CLPK_integer = 0;

                    ll::$ll(a.cols().as_const(),
                        a.sub_diagonals().as_const(), a.sup_diagonals().as_const(),
                        b.cols().as_const(),
                        a.as_mut_ptr().as_c_ptr(), a.rows().as_const(),
                        p.as_mut_ptr().as_c_ptr(),
                        b.as_mut_ptr().as_c_ptr(), b.rows().as_const(),
                        &mut info as *mut CLPK_integer);
                }
            }
        }
    );
)

gbsv_impl!(f32,         sgbsv_)
gbsv_impl!(f64,         dgbsv_)
gbsv_impl!(Complex32,   cgbsv_)
gbsv_impl!(Complex64,   zgbsv_)

pub trait Gtsv {
    fn gtsv(a: &mut TridiagonalMatrix<Self>, b: &mut Matrix<Self>);
}

macro_rules! gtsv_impl(
    ($t: ty, $ll: ident) => (
        impl Gtsv for $t {
            fn gtsv(a: &mut TridiagonalMatrix<$t>, b: &mut Matrix<$t>) {
                unsafe {
                    let mut info: CLPK_integer = 0;
                    let (sub, diag, sup) = a.as_mut_ptrs();

                    ll::$ll(a.cols().as_const(), b.cols().as_const(),
                        sub.as_c_ptr(), diag.as_c_ptr(), sup.as_c_ptr(),
                        b.as_mut_ptr().as_c_ptr(), b.rows().as_const(),
                        &mut info as *mut CLPK_integer);
                }
            }
        }
    );
)

gtsv_impl!(f32,         sgtsv_)
gtsv_impl!(f64,         dgtsv_)
gtsv_impl!(Complex32,   cgtsv_)
gtsv_impl!(Complex64,   zgtsv_)

pub trait Posv {
    fn posv(a: &mut SymmetricMatrix<Self>, b: &mut Matrix<Self>);
}

macro_rules! posv_impl(
    ($t: ty, $ll: ident) => (
        impl Posv for $t {
            fn posv(a: &mut SymmetricMatrix<$t>, b: &mut Matrix<$t>) {
                unsafe {
                    let mut info: CLPK_integer = 0;

                    ll::$ll(a.symmetry().as_i8().as_const(),
                        a.cols().as_const(), b.cols().as_const(),
                        a.as_mut_ptr().as_c_ptr(), a.rows().as_const(),
                        b.as_mut_ptr().as_c_ptr(), b.rows().as_const(),
                        &mut info as *mut CLPK_integer);
                }
            }
        }
    );
)

posv_impl!(f32,         sposv_)
posv_impl!(f64,         dposv_)
posv_impl!(Complex32,   cposv_)
posv_impl!(Complex64,   zposv_)
