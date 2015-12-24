// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.
use std::mem;
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
use types::Transpose;
use util::ColMem;

pub trait Gels: Sized {
    fn gels(a_trans: &Transpose, a: &mut Matrix<Self>, b: &mut Matrix<Self>) -> Result<(), Error> {
        //TODO: nancheck

        let work_len = try!(Gels::gels_work_len(a_trans, a, b));
        let mut work: Vec<_> = Vec::with_capacity(work_len as usize);
        unsafe {
            work.set_len(work_len as usize);
        }

        Gels::gels_work(a_trans, a, b, &mut work[..])
    }

    fn gels_work(a_trans: &Transpose, a: &mut Matrix<Self>, b: &mut Matrix<Self>, work: &mut [Self]) -> Result<(), Error>;
    fn gels_work_len(a_trans: &Transpose, a: &mut Matrix<Self>, b: &mut Matrix<Self>) -> Result<usize, Error>;
}

macro_rules! least_sq_impl(($($t: ident), +) => ($(
    impl Gels for $t {
        fn gels_work(a_trans: &Transpose, a: &mut Matrix<Self>, b: &mut Matrix<Self>, work: &mut [Self]) -> Result<(), Error> {
            let mut info: c_int = 0;

            let m = a.rows();
            let n = a.cols();
            let nrhs = b.cols();
            let mut a_mem = ColMem::new(a.order(), a);
            let mut b_mem = ColMem::new(b.order(), b);

            unsafe {
                prefix!($t, gels_)(
                    a_trans.as_i8().as_mut(),
                    m.as_mut(), n.as_mut(),
                    nrhs.as_mut(),
                    a_mem.as_mut_ptr(), a_mem.lead().as_mut(),
                    b_mem.as_mut_ptr(), b_mem.lead().as_mut(),
                    work.as_mut_ptr(), (work.len() as c_int).as_mut(),
                    &mut info as *mut c_int);
            }

            match info {
                0 => Ok(()),
                x if x < 0 => Err(Error::IllegalParameter(-x as usize)),
                x => Err(Error::DiagonalElementZero(x as usize)),
            }
        }

        fn gels_work_len(a_trans: &Transpose, a: &mut Matrix<Self>, b: &mut Matrix<Self>) -> Result<usize, Error> {
            let mut info: c_int = 0;
            let mut len_info: $t = unsafe { mem::zeroed() };
            let len_ptr = (&mut len_info) as *mut $t;

            let m = a.rows();
            let n = a.cols();
            let nrhs = b.cols();
            let lda_t = ::std::cmp::max(1, m);
            let ldb_t = ::std::cmp::max(1, ::std::cmp::max(m, n));

            unsafe {
                prefix!($t, gels_)(
                    a_trans.as_i8().as_mut(),
                    m.as_mut(), n.as_mut(),
                    nrhs.as_mut(),
                    a.as_mut_ptr(), lda_t.as_mut(),
                    b.as_mut_ptr(), ldb_t.as_mut(),
                    len_ptr, (-1 as c_int).as_mut(),
                    &mut info as *mut c_int);
            };

            match info {
                0 => Ok(len_info.as_work()),
                x if x < 0 => Err(Error::IllegalParameter(-x as usize)),
                x => Err(Error::DiagonalElementZero(x as usize)),
            }
        }
    }
)+));

least_sq_impl!(f32, f64, Complex32, Complex64);

#[cfg(test)]
mod gels_tests {
    use types::Order::*;
    use types::Transpose;
    use matrix::tests::M;
    use least_squares::Gels;

    #[test]
    fn col_major() {
        let mut a = M(ColMajor, 3i32, 2i32, vec![2.0f32,4.0,7.0,3.0,9.0,4.0]);
        let mut b = M(ColMajor, 3i32, 2i32, vec![2.0f32,4.0,7.0,6.0,18.0,8.0]);

        Gels::gels(&Transpose::None, &mut a, &mut b).unwrap();

        let M(_, _, _, x) = b;
        assert_eq!(x, vec![1.0, 0.0, 0.0, 0.0, 2.0, 0.0]);
    }

    #[test]
    fn row_major() {
        let mut a = M(RowMajor, 3i32, 2i32, vec![2.0f32,3.0,4.0,9.0,7.0,4.0]);
        let mut b = M(RowMajor, 3i32, 2i32, vec![2.0f32,3.0,4.0,9.0,7.0,4.0]);

        Gels::gels(&Transpose::None, &mut a, &mut b).unwrap();

        let M(_, _, _, x) = b;
        assert_eq!(x, vec![1.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
    }
}
