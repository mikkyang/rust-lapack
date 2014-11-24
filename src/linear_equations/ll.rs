// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

extern crate libc;
use self::libc::{
    c_int,
};

use types::{
    CLPK_complex,
    CLPK_doublecomplex,
    CLPK_doublereal,
    CLPK_integer,
    CLPK_real,
};

#[link(name = "lapack")]
extern "C" {
    pub fn sgesv_(n: *const CLPK_integer, nrhs: *const CLPK_integer, a: *mut CLPK_real,
            lda: *const CLPK_integer, ipiv: *mut CLPK_integer, b: *mut CLPK_real,
            ldb: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn dgesv_(n: *const CLPK_integer, nrhs: *const CLPK_integer, a: *mut CLPK_doublereal,
            lda: *const CLPK_integer, ipiv: *mut CLPK_integer, b: *mut CLPK_doublereal,
            ldb: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn cgesv_(n: *const CLPK_integer, nrhs: *const CLPK_integer, a: *mut CLPK_complex,
            lda: *const CLPK_integer, ipiv: *mut CLPK_integer, b: *mut CLPK_complex,
            ldb: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn zgesv_(n: *const CLPK_integer, nrhs: *const CLPK_integer,
            a: *mut CLPK_doublecomplex, lda: *const CLPK_integer,
            ipiv: *mut CLPK_integer, b: *mut CLPK_doublecomplex,
            ldb: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;

    pub fn sgbsv_(n: *const CLPK_integer, kl: *const CLPK_integer, ku: *const CLPK_integer,
            nrhs: *const CLPK_integer, ab: *mut CLPK_real, ldab: *const CLPK_integer,
            ipiv: *mut CLPK_integer, b: *mut CLPK_real, ldb: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn dgbsv_(n: *const CLPK_integer, kl: *const CLPK_integer, ku: *const CLPK_integer,
            nrhs: *const CLPK_integer, ab: *mut CLPK_doublereal, ldab: *const CLPK_integer,
            ipiv: *mut CLPK_integer, b: *mut CLPK_doublereal, ldb: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn cgbsv_(n: *const CLPK_integer, kl: *const CLPK_integer, ku: *const CLPK_integer,
            nrhs: *const CLPK_integer, ab: *mut CLPK_complex, ldab: *const CLPK_integer,
            ipiv: *mut CLPK_integer, b: *mut CLPK_complex, ldb: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn zgbsv_(n: *const CLPK_integer, kl: *const CLPK_integer, ku: *const CLPK_integer,
            nrhs: *const CLPK_integer, ab: *mut CLPK_doublecomplex,
            ldab: *const CLPK_integer, ipiv: *mut CLPK_integer,
            b: *mut CLPK_doublecomplex, ldb: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
}
