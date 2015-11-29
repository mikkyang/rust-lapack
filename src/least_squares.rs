// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.
use std::cmp;
use std::mem;
use std::slice;
use libc::c_int;
use num::complex::{
    Complex32,
    Complex64,
};
use error::Error;
use ll::*;
use matrix::{
    Matrix,
};
use scalar::Scalar;
use types::Layout;
use util::transpose_data;

pub trait Gels {
    fn gels(a: &mut Matrix<Self>, b: &mut Matrix<Self>) -> Result<(), Error>;
    fn gels_work(a: &mut Matrix<Self>, b: &mut Matrix<Self>, work: &mut [Self]) -> Result<(), Error>;
    fn gels_work_len(a: &mut Matrix<Self>, b: &mut Matrix<Self>) -> Result<usize, Error>;
}

macro_rules! least_sq_impl(($($t: ident), +) => ($(
    impl Gels for $t {
        fn gels(a: &mut Matrix<Self>, b: &mut Matrix<Self>) -> Result<(), Error> {
            //TODO: nancheck

            let mut info: c_int = 0;
            let work_len = try!(Gels::gels_work_len(a, b));
            let mut work: Vec<$t> = Vec::with_capacity(work_len as usize);

            unsafe {
                work.set_len(work_len as usize);
            }

            Gels::gels_work(a, b, &mut work[..])
        }

        fn gels_work(a: &mut Matrix<Self>, b: &mut Matrix<Self>, work: &mut [Self]) -> Result<(), Error> {
            let mut info: c_int = 0;

            let m = a.rows();
            let n = a.cols();
            let mrhs = cmp::max(m, n);
            let nrhs = b.cols();

            let mut a_t: Option<Vec<$t>> = None;
            let mut b_t: Option<Vec<$t>> = None;

            let (a_ptr, lda) = match a.layout() {
                Layout::ColMajor => (a.as_mut_ptr(), a.rows()),
                Layout::RowMajor => {
                    let lda_t = cmp::max(1, m);
                    let len = (lda_t * cmp::max(1, n)) as usize;
                    let mut temp = Vec::with_capacity(len);
                    unsafe {
                        temp.set_len(len);
                    }
                    let in_slice = unsafe {
                        slice::from_raw_parts(a.as_ptr(), (m * n) as usize)
                    };

                    transpose_data(Layout::RowMajor, m as usize, n as usize, in_slice, m as usize, &mut temp[..], lda_t as usize);

                    a_t = Some(temp);
                    (a_t.unwrap().as_mut_ptr(), lda_t)
                },
            };

            let (b_ptr, ldb) = match b.layout() {
                Layout::ColMajor => (b.as_mut_ptr(), b.rows()),
                Layout::RowMajor => {
                    let ldb_t = cmp::max(1, mrhs);
                    let len = (ldb_t * cmp::max(1, nrhs)) as usize;
                    let mut temp = Vec::with_capacity(len);
                    unsafe {
                        temp.set_len(len);
                    }
                    let in_slice = unsafe {
                        slice::from_raw_parts(b.as_ptr(), (mrhs * nrhs) as usize)
                    };

                    transpose_data(Layout::RowMajor, mrhs as usize, nrhs as usize, in_slice, mrhs as usize, &mut temp[..], ldb_t as usize);

                    b_t = Some(temp);
                    (b_t.unwrap().as_mut_ptr(), ldb_t)
                },
            };

            unsafe {
                prefix!($t, gels_)(
                    a.transpose().as_i8().as_mut(),
                    m.as_mut(), n.as_mut(),
                    nrhs.as_mut(),
                    a_ptr, lda.as_mut(),
                    b_ptr, ldb.as_mut(),
                    work.as_mut_ptr(), (work.len() as c_int).as_mut(),
                    &mut info as *mut c_int);
            };

            match info {
                0 => Ok(()),
                x if x < 0 => Err(Error::IllegalParameter(-x as usize)),
                x => Err(Error::DiagonalElementZero(x as usize)),
            }
        }

        fn gels_work_len(a: &mut Matrix<Self>, b: &mut Matrix<Self>) -> Result<usize, Error> {
            let mut info: c_int = 0;
            let mut len_info: [$t; 1] = unsafe { mem::zeroed() };
            let len_ptr = (&mut len_info).as_mut_ptr();

            let m = a.rows();
            let n = a.cols();
            let nrhs = b.cols();
            let lda_t = cmp::max(1, m);
            let ldb_t = cmp::max(1, cmp::max(m, n));

            unsafe {
                prefix!($t, gels_)(
                    a.transpose().as_i8().as_mut(),
                    m.as_mut(), n.as_mut(),
                    nrhs.as_mut(),
                    a.as_mut_ptr(), lda_t.as_mut(),
                    b.as_mut_ptr(), ldb_t.as_mut(),
                    len_ptr, (-1 as c_int).as_mut(),
                    &mut info as *mut c_int);
            };

            match info {
                0 => unsafe {
                    let cast_ptr = len_ptr as *const c_int;
                    Ok(*cast_ptr as usize)
                },
                x if x < 0 => Err(Error::IllegalParameter(-x as usize)),
                x => Err(Error::DiagonalElementZero(x as usize)),
            }
        }
    }
)+));

least_sq_impl!(f32, f64, Complex32, Complex64);
