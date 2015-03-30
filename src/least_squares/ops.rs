// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.
use std::cmp;
use num::complex::{
    Complex32,
    Complex64,
};
use least_squares::ll::*;
use matrix::{
    Matrix,
};
use pointer::CPtr;
use scalar::Scalar;
use types::{
    CLPK_integer,
};

pub trait Gels {
    fn gels(a: &mut Matrix<Self>, b: &mut Matrix<Self>);
}

macro_rules! least_sq_impl(($($t: ident), +) => ($(
    impl Gels for $t {
        fn gels(a: &mut Matrix<Self>, b: &mut Matrix<Self>) {
            unsafe {
                let mut info: CLPK_integer = 0;

                let m = a.rows();
                let n = a.cols();
                let nrhs = b.cols();
                let mn = cmp::min(m, n);
                let work_len = mn + cmp::max(mn, nrhs);
                let mut work: Vec<$t> = Vec::with_capacity(work_len as usize);
                work.set_len(work_len as usize);

                prefix!($t, gels_)(a.transpose().as_i8().as_const(),
                    m.as_const(), n.as_const(),
                    nrhs.as_const(),
                    a.as_mut_ptr().as_c_ptr(), a.rows().as_const(),
                    b.as_mut_ptr().as_c_ptr(), b.rows().as_const(),
                    (&mut work[..]).as_mut_ptr().as_c_ptr(), work_len.as_const(),
                    &mut info as *mut CLPK_integer);
            }
        }
    }
)+));

least_sq_impl!(f32, f64, Complex32, Complex64);
