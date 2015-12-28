// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.
#[cfg(not(feature = "default"))]
use libc::c_int;

#[cfg(feature = "default")]
pub use rblas::vector::Vector;

#[cfg(not(feature = "default"))]
/// Methods that allow a type to be used in BLAS functions as a vector.
pub trait Vector<T> {
    /// The stride within the vector. For example, if `inc` returns 7, every
    /// 7th element is used. Defaults to 1.
    fn inc(&self) -> c_int { 1 }
    /// The number of elements in the vector.
    fn len(&self) -> c_int;
    /// An unsafe pointer to a contiguous block of memory.
    fn as_ptr(&self) -> *const T;
    /// An unsafe mutable pointer to a contiguous block of memory.
    fn as_mut_ptr(&mut self) -> *mut T;
}
