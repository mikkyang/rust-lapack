// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

extern crate num;

use self::num::complex::{
    Complex32,
    Complex64,
};
use linear_equations::ll;
use matrix::Matrix;
use pointer::CPtr;
use scalar::Scalar;
use types::{
    CLPK_integer,
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
