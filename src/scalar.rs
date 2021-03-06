// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.
use num::complex::Complex;
use libc::{
    c_char,
    c_double,
    c_float,
    c_int,
    c_void,
};

pub trait Scalar<T, S> {
    fn as_const(self) -> T;
    fn as_mut(self) -> S;
    fn as_work(self) -> usize;
}

macro_rules! scalar_impl(
    ($t: ty, $c_type: ty) => (
        impl<'a> Scalar<*const $t, *mut $t> for &'a $t {
            #[inline]
            fn as_const(self) -> *const $t {
                self as *const $c_type
            }

            #[inline]
            fn as_mut(self) -> *mut $t {
                self as *const _ as *mut $c_type
            }

            #[inline]
            fn as_work(self) -> usize {
                *self as usize
            }
        }

        impl<'a> Scalar<*const c_void, *mut c_void> for &'a Complex<$t> {
            #[inline]
            fn as_const(self) -> *const c_void {
                self as *const _ as *const c_void
            }

            #[inline]
            fn as_mut(self) -> *mut c_void {
                self as *const _ as *mut c_void
            }

            #[inline]
            fn as_work(self) -> usize {
                self.re as usize
            }
        }
    );
);

scalar_impl!(i8, c_char);
scalar_impl!(i32, c_int);
scalar_impl!(f32, c_float);
scalar_impl!(f64, c_double);
