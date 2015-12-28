// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.
use std::mem;
use std::ptr;
use libc::c_int;
use num::complex::{
    Complex,
};
use error::Error;
use ll::*;
use Matrix;
use Vector;
use scalar::Scalar;
use types::Compute;
use util::ColMem;

pub trait Geev<Eigenvalues>: Sized {
    fn geev(a: &mut Matrix<Self>, values: &mut Vector<Eigenvalues>,
        left: Option<&mut Matrix<Self>>, right: Option<&mut Matrix<Self>>) -> Result<(), Error> {

        let job_l = match &left {
            &Some(_) => Compute::Value,
            _ => Compute::None,
        };

        let job_r = match &right {
            &Some(_) => Compute::Value,
            _ => Compute::None,
        };

        let work_len = try!(Geev::work_len(a, job_l, job_r));
        let mut work: Vec<_> = Vec::with_capacity(work_len as usize);
        unsafe {
            work.set_len(work_len as usize);
        }

        Geev::work(a, values, left, right, &mut work[..])
    }
    fn work(a: &mut Matrix<Self>, values: &mut Vector<Eigenvalues>,
        left: Option<&mut Matrix<Self>>, right: Option<&mut Matrix<Self>>, work: &mut [Self]) -> Result<(), Error>;
    fn work_len(a: &mut Matrix<Self>, left: Compute, right: Compute) -> Result<usize, Error>;
}

macro_rules! real_eigen_impl(($($t: ident), +) => ($(
    impl Geev<Complex<$t>> for $t {
        fn work(a: &mut Matrix<Self>, values: &mut Vector<Complex<$t>>,
            left: Option<&mut Matrix<Self>>, right: Option<&mut Matrix<Self>>, work: &mut [Self]) -> Result<(), Error> {

            let mut info: c_int = 0;
            let n = a.rows();
            if n != a.cols() {
                return Err(Error::DimensionMismatch);
            }
            let mut a_mem = ColMem::new(a.order(), a);

            let (job_l, lead_l, ptr_l) = match left {
                Some(m) => (Compute::Value, m.rows(), m.as_mut_ptr()),
                None => (Compute::None, 1, ptr::null::<$t>() as *mut _),
            };

            let (job_r, lead_r, ptr_r) = match right {
                Some(m) => (Compute::Value, m.rows(), m.as_mut_ptr()),
                None => (Compute::None, 1, ptr::null::<$t>() as *mut _),
            };

            //TODO: this is really bad
            let (real, imag) = unsafe {
                let start = values.as_mut_ptr();
                let mid = start.offset(n as isize);
                (start as *mut $t, mid as *mut $t)
            };

            unsafe {
                prefix!($t, geev_)(
                    job_l.as_i8().as_mut(), job_r.as_i8().as_mut(),
                    n.as_mut(),
                    a_mem.as_mut_ptr(), a_mem.lead().as_mut(),
                    real, imag,
                    ptr_l, lead_l.as_mut(),
                    ptr_r, lead_r.as_mut(),
                    work.as_mut_ptr(), (work.len() as c_int).as_mut(),
                    &mut info as *mut c_int);
            };

            match info {
                0 => Ok(()),
                x if x < 0 => Err(Error::IllegalParameter(-x as usize)),
                x => Err(Error::DiagonalElementZero(x as usize)),
            }
        }

        fn work_len(a: &mut Matrix<Self>, left: Compute, right: Compute) -> Result<usize, Error> {
            let mut info: c_int = 0;
            let mut len_info: $t = unsafe { mem::zeroed() };
            let len_ptr = (&mut len_info) as *mut $t;

            let n = a.rows();
            if n != a.cols() {
                return Err(Error::DimensionMismatch);
            }
            let lda = n;

            unsafe {
                prefix!($t, geev_)(
                    left.as_i8().as_mut(), right.as_i8().as_mut(),
                    n.as_mut(),
                    a.as_mut_ptr(), lda.as_mut(),
                    ptr::null::<$t>() as *mut _, ptr::null::<$t>() as *mut _,
                    ptr::null::<$t>() as *mut _, n.as_mut(),
                    ptr::null::<$t>() as *mut _, n.as_mut(),
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

real_eigen_impl!(f32, f64);

#[cfg(test)]
mod geev_tests {
    use eigenvalues::Geev;
    use matrix::tests::M;
    use types::Order::*;

    #[test]
    fn real() {
        let mut a = M(RowMajor, 2i32, 2i32, vec![2.0f64, 7.0, -1.0, -6.0]);
        let mut v = Vec::with_capacity(2);
        unsafe { v.set_len(2) };

        Geev::geev(&mut a, &mut v, None, None).unwrap();

        assert_eq!(v, vec![]);
    }
}

