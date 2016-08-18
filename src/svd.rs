use std::mem;
use std::ptr;
use libc::c_int;
use error::Error;
use ll::*;
use Matrix;
use Vector;
use scalar::Scalar;
use types::Compute;
use util::ColMem;

pub trait Gesvd<Eigenvalues>: Sized {
    fn gesvd(a: &mut Matrix<Self>, left: Option<&mut Matrix<Self>>, right: Option<&mut Matrix<Self>>) -> Result<Vec<Eigenvalues>, Error> {

        let job_l = match &left {
            &Some(_) => Compute::Some,
            _ => Compute::None,
        };

        let job_r = match &right {
            &Some(_) => Compute::Some,
            _ => Compute::None,
        };

        let work_len = try!(Gesvd::work_len(a, job_l, job_r));
        let mut work: Vec<_> = Vec::with_capacity(work_len as usize);
        unsafe {
            work.set_len(work_len as usize);
        }

        Gesvd::work(a, left, right, &mut work[..])
    }
    fn work(a: &mut Matrix<Self>,
        left: Option<&mut Matrix<Self>>, right: Option<&mut Matrix<Self>>, work: &mut [Self]) -> Result<Vec<Eigenvalues>, Error>;
    fn work_len(a: &mut Matrix<Self>, left: Compute, right: Compute) -> Result<usize, Error>;
}

macro_rules! real_svd_impl(($($t: ident), +) => ($(
    impl Gesvd<$t> for $t {
        fn work(a: &mut Matrix<Self>, left: Option<&mut Matrix<Self>>, right: Option<&mut Matrix<Self>>, work: &mut [Self]) -> Result<Vec<$t>, Error> {

            let mut info: c_int = 0;
            let m = a.rows();
            let n = a.cols();

            if m < n {
                return Err(Error::DimensionMismatch);
            }

            let mut a_mem = ColMem::new(a.order(), a);

            let (job_l, lead_l, ptr_l) = match left {
                Some(m) => (Compute::Some, m.rows(), m.as_mut_ptr()),
                None => (Compute::None, 1, ptr::null::<$t>() as *mut _),
            };

            let (job_r, lead_r, ptr_r) = match right {
                Some(m) => (Compute::Some, m.rows(), m.as_mut_ptr()),
                None => (Compute::None, 1, ptr::null::<$t>() as *mut _),
            };

            let mut singular_values: Vec<_> = Vec::with_capacity(n as usize);

            unsafe {
                singular_values.set_len(n as usize);

                prefix!($t, gesvd_)(
                    job_l.as_i8().as_mut(), job_r.as_i8().as_mut(),
                    m.as_mut(), n.as_mut(),
                    a_mem.as_mut_ptr(),
                    a_mem.lead().as_mut(),
                    singular_values.as_mut_ptr(),
                    ptr_l, lead_l.as_mut(),
                    ptr_r, lead_r.as_mut(),
                    work.as_mut_ptr(), (work.len() as c_int).as_mut(),
                    &mut info as *mut c_int);
            };

            match info {
                0 => Ok(singular_values),
                x if x < 0 => Err(Error::IllegalParameter(-x as usize)),
                x => Err(Error::DiagonalElementZero(x as usize)),
            }
        }

        fn work_len(a: &mut Matrix<Self>, left: Compute, right: Compute) -> Result<usize, Error> {
            let mut info: c_int = 0;
            let mut len_info: $t = unsafe { mem::zeroed() };
            let len_ptr = (&mut len_info) as *mut $t;

            let m = a.rows();
            let n = a.cols();
            let lda = m;

            unsafe {
                prefix!($t, gesvd_)(
                    left.as_i8().as_mut(), right.as_i8().as_mut(),
                    m.as_mut(), n.as_mut(),
                    a.as_mut_ptr(), lda.as_mut(),
                    ptr::null::<$t>() as *mut _, // S
                    ptr::null::<$t>() as *mut _, // U
                    m.as_mut(),                  // LDU
                    ptr::null::<$t>() as *mut _, // VT
                    n.as_mut(),                  // LDVT
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

real_svd_impl!(f32, f64);

#[cfg(test)]
mod gesvd_tests {
    use svd::Gesvd;
    use matrix::tests::M;
    use types::Order::*;

    #[test]
    fn real() {
        let mut a = M(RowMajor, 3i32, 2i32, vec![3.0f32, 2.0, 2.0, 3.0, 2.0, -2.0]);
        let mut v = M(RowMajor, 2i32, 2i32, vec![0.0, 0.0, 0.0, 0.0]);
        let lambda = Gesvd::gesvd(&mut a, None, Some(&mut v)).unwrap();

        assert!((lambda[0] - 5.0).abs() < 0.0000001);
        assert!((lambda[1] - 3.0).abs() < 0.001);
        assert_eq!(v.3[0], 0.0);
    }
}
