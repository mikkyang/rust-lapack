// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.
use std::cmp;
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
use types::Layout;
use util::transpose_data;

pub trait Gels: Sized {
    fn gels(layout: Layout, a: &mut Matrix<Self>, b: &mut Matrix<Self>) -> Result<(), Error>;
    fn gels_work(layout: Layout, a: &mut Matrix<Self>, b: &mut Matrix<Self>, work: &mut [Self]) -> Result<(), Error>;
    fn gels_work_len(a: &mut Matrix<Self>, b: &mut Matrix<Self>) -> Result<usize, Error>;
}

macro_rules! least_sq_impl(($($t: ident), +) => ($(
    impl Gels for $t {
        fn gels(layout: Layout, a: &mut Matrix<Self>, b: &mut Matrix<Self>) -> Result<(), Error> {
            //TODO: nancheck

            let work_len = try!(Gels::gels_work_len(a, b));
            let mut work: Vec<$t> = Vec::with_capacity(work_len as usize);
            unsafe {
                work.set_len(work_len as usize);
            }

            Gels::gels_work(layout, a, b, &mut work[..])
        }

        fn gels_work(layout: Layout, a: &mut Matrix<Self>, b: &mut Matrix<Self>, work: &mut [Self]) -> Result<(), Error> {
            let mut info: c_int = 0;

            let m = a.rows();
            let n = a.cols();
            let nrhs = b.cols();

            match layout {
                Layout::ColMajor => unsafe {
                    let lda = m;
                    let ldb = b.rows();

                    prefix!($t, gels_)(
                        a.transpose().as_i8().as_mut(),
                        m.as_mut(), n.as_mut(),
                        nrhs.as_mut(),
                        a.as_mut_ptr(), lda.as_mut(),
                        b.as_mut_ptr(), ldb.as_mut(),
                        work.as_mut_ptr(), (work.len() as c_int).as_mut(),
                        &mut info as *mut c_int);
                },
                Layout::RowMajor => {
                    let lda = n;
                    let ldb = nrhs;
                    let mrhs = cmp::max(m, n);

                    let lda_t = cmp::max(1, m);
                    let ldb_t = cmp::max(1, mrhs);

                    let a_t_len = (lda_t * cmp::max(1, n)) as usize;
                    let b_t_len = (ldb_t * cmp::max(1, nrhs)) as usize;
                    let mut a_t = Vec::with_capacity(a_t_len);
                    let mut b_t = Vec::with_capacity(b_t_len);

                    unsafe {
                        a_t.set_len(a_t_len);
                        b_t.set_len(b_t_len);

                        transpose_data(Layout::RowMajor, m as isize, n as isize, a.as_ptr(), lda as isize, a_t.as_mut_ptr(), lda_t as isize);
                        transpose_data(Layout::RowMajor, mrhs as isize, nrhs as isize, b.as_ptr(), ldb as isize, b_t.as_mut_ptr(), ldb_t as isize);

                        prefix!($t, gels_)(
                            a.transpose().as_i8().as_mut(),
                            m.as_mut(), n.as_mut(),
                            nrhs.as_mut(),
                            a_t.as_mut_ptr(), lda_t.as_mut(),
                            b_t.as_mut_ptr(), ldb_t.as_mut(),
                            work.as_mut_ptr(), (work.len() as c_int).as_mut(),
                            &mut info as *mut c_int);

                        transpose_data(Layout::ColMajor, m as isize, n as isize, a_t.as_ptr(), lda_t as isize, a.as_mut_ptr(), lda as isize);
                        transpose_data(Layout::ColMajor, mrhs as isize, nrhs as isize, b_t.as_ptr(), ldb_t as isize, b.as_mut_ptr(), ldb as isize);
                    }
                }
            }

            match info {
                0 => Ok(()),
                x if x < 0 => Err(Error::IllegalParameter(-x as usize)),
                x => Err(Error::DiagonalElementZero(x as usize)),
            }
        }

        fn gels_work_len(a: &mut Matrix<Self>, b: &mut Matrix<Self>) -> Result<usize, Error> {
            let mut info: c_int = 0;
            let mut len_info: $t = unsafe { mem::zeroed() };
            let len_ptr = (&mut len_info) as *mut $t;

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
                0 => Ok(len_info.as_work()),
                x if x < 0 => Err(Error::IllegalParameter(-x as usize)),
                x => Err(Error::DiagonalElementZero(x as usize)),
            }
        }
    }
)+));

least_sq_impl!(f32, f64, Complex32, Complex64);

#[cfg(test)]
mod gesv_tests {
    use types::Layout::*;
    use least_squares::Gels;

    #[test]
    fn col_major() {
        let mut a = (3i32, 2i32, vec![2.0f32,4.0,7.0,3.0,9.0,4.0]);
        let mut b = (3i32, 2i32, vec![2.0f32,4.0,7.0,6.0,18.0,8.0]);

        Gels::gels(ColMajor, &mut a, &mut b).unwrap();

        let (_, _, x) = b;
        assert_eq!(x, vec![1.0, 0.0, 0.0, 0.0, 2.0, 0.0]);
    }

    #[test]
    fn row_major() {
        let mut a = (3i32, 2i32, vec![2.0f32,3.0,4.0,9.0,7.0,4.0]);
        let mut b = (3i32, 2i32, vec![2.0f32,3.0,4.0,9.0,7.0,4.0]);

        Gels::gels(RowMajor, &mut a, &mut b).unwrap();

        let (_, _, x) = b;
        assert_eq!(x, vec![1.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
    }
}
