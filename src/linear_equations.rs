// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.
use num::complex::{
    Complex32,
    Complex64,
};
use libc::c_int;
use error::Error;
use ll::*;
use matrix::{
    Matrix,
    BandMatrix,
    SymmetricMatrix,
    TridiagonalMatrix,
};
use scalar::Scalar;
use types::Symmetry;
use util::ColMem;

pub trait Gesv {
    fn gesv(a: &mut Matrix<Self>, b: &mut Matrix<Self>)
        -> Result<Vec<usize>, Error>;
}

pub trait Gbsv {
    fn gbsv(a: &mut BandMatrix<Self>, b: &mut Matrix<Self>, p: &mut Matrix<c_int>);
}

pub trait Gtsv {
    fn gtsv(a: &mut TridiagonalMatrix<Self>, b: &mut Matrix<Self>);
}

pub trait Posv {
    fn posv(a: &mut SymmetricMatrix<Self>, b: &mut Matrix<Self>);
}

pub trait Ppsv {
    fn ppsv(a: &mut SymmetricMatrix<Self>, b: &mut Matrix<Self>);
}

pub trait Pbsv<M>: Sized where M: SymmetricMatrix<Self> + BandMatrix<Self> {
    fn pbsv(a: &mut M, b: &mut Matrix<Self>);
}

pub trait Ptsv<M>: Sized where M: TridiagonalMatrix<Self> + SymmetricMatrix<Self> {
    fn ptsv(a: &mut M, b: &mut Matrix<Self>);
}

pub trait Sysv {
    fn sysv(a: &mut SymmetricMatrix<Self>, b: &mut Matrix<Self>, p: &mut Matrix<c_int>);
}

pub trait Hesv {
    fn hesv(a: &mut SymmetricMatrix<Self>, b: &mut Matrix<Self>, p: &mut Matrix<c_int>);
}

pub trait Spsv {
    fn spsv(a: &mut SymmetricMatrix<Self>, b: &mut Matrix<Self>, p: &mut Matrix<c_int>);
}

pub trait Hpsv {
    fn hpsv(a: &mut SymmetricMatrix<Self>, b: &mut Matrix<Self>, p: &mut Matrix<c_int>);
}

macro_rules! lin_eq_impl(($($t: ident), +) => ($(
    impl Gesv for $t {
        fn gesv(a: &mut Matrix<$t>, b: &mut Matrix<$t>) -> Result<Vec<usize>, Error> {
            //TODO: nancheck

            let mut info: c_int = 0;

            let n = a.rows();
            if n != a.cols() {
                return Err(Error::DimensionMismatch);
            }

            let nrhs = b.cols();
            let mut pivot_indices: Vec<usize> = Vec::with_capacity(n as usize);
            unsafe { pivot_indices.set_len(n as usize); }

            let mut a_mem = ColMem::new(a.order(), a);
            let mut b_mem = ColMem::new(b.order(), b);

            unsafe {
                prefix!($t, gesv_)(
                    n.as_mut(), nrhs.as_mut(),
                    a_mem.as_mut_ptr(), a_mem.lead().as_mut(),
                    pivot_indices.as_mut_ptr() as *mut c_int,
                    b_mem.as_mut_ptr(), b_mem.lead().as_mut(),
                    &mut info as *mut c_int);
            }

            match info {
                0 => Ok(pivot_indices),
                x if x < 0 => Err(Error::IllegalParameter(-x as usize)),
                x => Err(Error::DiagonalElementZero(x as usize)),
            }
        }
    }

    impl Gbsv for $t {
        fn gbsv(a: &mut BandMatrix<$t>, b: &mut Matrix<$t>, p: &mut Matrix<c_int>) {
            unsafe {
                let mut info: c_int = 0;

                prefix!($t, gbsv_)(a.cols().as_mut(),
                    a.sub_diagonals().as_mut(), a.sup_diagonals().as_mut(),
                    b.cols().as_mut(),
                    a.as_mut_ptr(), a.rows().as_mut(),
                    p.as_mut_ptr(),
                    b.as_mut_ptr(), b.rows().as_mut(),
                    &mut info as *mut c_int);
            }
        }
    }

    impl Gtsv for $t {
        fn gtsv(a: &mut TridiagonalMatrix<Self>, b: &mut Matrix<Self>) {
            unsafe {
                let mut info: c_int = 0;

                let (sup, diag, sub) = a.as_mut_ptrs();

                prefix!($t, gtsv_)(a.cols().as_mut(), b.cols().as_mut(),
                    sup, diag, sub,
                    b.as_mut_ptr(), b.rows().as_mut(),
                    &mut info as *mut c_int);
            }
        }
    }

    impl Posv for $t {
        fn posv(a: &mut SymmetricMatrix<$t>, b: &mut Matrix<$t>) {
            unsafe {
                let mut info: c_int = 0;

                prefix!($t, posv_)(a.symmetry().as_i8().as_mut(),
                    a.cols().as_mut(), b.cols().as_mut(),
                    a.as_mut_ptr(), a.rows().as_mut(),
                    b.as_mut_ptr(), b.rows().as_mut(),
                    &mut info as *mut c_int);
            }
        }
    }

    impl Ppsv for $t {
        fn ppsv(a: &mut SymmetricMatrix<$t>, b: &mut Matrix<$t>) {
            unsafe {
                let mut info: c_int = 0;

                prefix!($t, ppsv_)(a.symmetry().as_i8().as_mut(),
                    a.cols().as_mut(), b.cols().as_mut(),
                    a.as_mut_ptr(),
                    b.as_mut_ptr(), b.rows().as_mut(),
                    &mut info as *mut c_int);
            }
        }
    }

    impl<M> Pbsv<M> for $t where M: SymmetricMatrix<$t> + BandMatrix<$t> {
        fn pbsv(a: &mut M, b: &mut Matrix<$t>) {
            unsafe {
                let mut info: c_int = 0;

                let diag = match a.symmetry() {
                    Symmetry::Upper => a.sup_diagonals(),
                    Symmetry::Lower => a.sub_diagonals(),
                };

                prefix!($t, pbsv_)(a.symmetry().as_i8().as_mut(),
                    a.cols().as_mut(), diag.as_mut(),
                    b.cols().as_mut(),
                    a.as_mut_ptr(), a.rows().as_mut(),
                    b.as_mut_ptr(), b.rows().as_mut(),
                    &mut info as *mut c_int);
            }
        }
    }

    impl Sysv for $t {
        fn sysv(a: &mut SymmetricMatrix<$t>, b: &mut Matrix<$t>, p: &mut Matrix<c_int>) {
            unsafe {
                let mut info: c_int = 0;

                let n = a.cols() as usize;
                let mut work: Vec<$t> = Vec::with_capacity(n);

                prefix!($t, sysv_)(a.symmetry().as_i8().as_mut(),
                    a.cols().as_mut(), b.cols().as_mut(),
                    a.as_mut_ptr(), a.rows().as_mut(),
                    p.as_mut_ptr(),
                    b.as_mut_ptr(), b.rows().as_mut(),
                    (&mut work[..]).as_mut_ptr(), a.cols().as_mut(),
                    &mut info as *mut c_int);
            }
        }
    }

    impl Spsv for $t {
        fn spsv(a: &mut SymmetricMatrix<Self>, b: &mut Matrix<Self>, p: &mut Matrix<c_int>) {
            unsafe {
                let mut info: c_int = 0;

                prefix!($t, spsv_)(a.symmetry().as_i8().as_mut(),
                    a.cols().as_mut(), b.cols().as_mut(),
                    a.as_mut_ptr(),
                    p.as_mut_ptr(),
                    b.as_mut_ptr(), b.rows().as_mut(),
                    &mut info as *mut c_int);
            }
        }
    }
)+));

macro_rules! complex_lin_eq_impl(($($t: ident), +) => ($(
    impl Hesv for $t {
        fn hesv(a: &mut SymmetricMatrix<$t>, b: &mut Matrix<$t>, p: &mut Matrix<c_int>) {
            unsafe {
                let mut info: c_int = 0;

                let n = a.cols() as usize;
                let mut work: Vec<$t> = Vec::with_capacity(n);

                prefix!($t, hesv_)(a.symmetry().as_i8().as_mut(),
                    a.cols().as_mut(), b.cols().as_mut(),
                    a.as_mut_ptr(), a.rows().as_mut(),
                    p.as_mut_ptr(),
                    b.as_mut_ptr(), b.rows().as_mut(),
                    (&mut work[..]).as_mut_ptr(), a.cols().as_mut(),
                    &mut info as *mut c_int);
            }
        }
    }

    impl Hpsv for $t {
        fn hpsv(a: &mut SymmetricMatrix<Self>, b: &mut Matrix<Self>, p: &mut Matrix<c_int>) {
            unsafe {
                let mut info: c_int = 0;

                prefix!($t, hpsv_)(a.symmetry().as_i8().as_mut(),
                    a.cols().as_mut(), b.cols().as_mut(),
                    a.as_mut_ptr(),
                    p.as_mut_ptr(),
                    b.as_mut_ptr(), b.rows().as_mut(),
                    &mut info as *mut c_int);
            }
        }
    }
)+));

lin_eq_impl!(f32, f64, Complex32, Complex64);
complex_lin_eq_impl!(Complex32, Complex64);

#[cfg(test)]
mod gesv_tests {
    use linear_equations::Gesv;
    use matrix::tests::M;
    use types::Order::*;

    #[test]
    fn real() {
        let mut a = M(ColMajor, 2i32, 2i32, vec![1.0f64, 4.0, 1.0, 2.0]);
        let mut b = M(ColMajor, 2i32, 1i32, vec![-2.0f64, 2.0]);

        Gesv::gesv(&mut a, &mut b).unwrap();

        let M(_, _, _, x) = b;
        assert_eq!(x, vec![3.0f64, -5.0]);
    }
}
