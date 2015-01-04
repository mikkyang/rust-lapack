// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.
use libc::{
    c_char,
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
    pub fn ssygv_(itype: CLPK_integer, jobz: c_char, uplo: c_char,
            n: *const CLPK_integer, a: *mut CLPK_real, lda: *const CLPK_integer,
            b: *mut CLPK_real, ldb: *const CLPK_integer, w: CLPK_real,
            work: CLPK_real, lwork: CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn dsygv_(itype: CLPK_integer, jobz: c_char, uplo: c_char,
            n: *const CLPK_integer, a: *mut CLPK_doublereal, lda: *const CLPK_integer,
            b: *mut CLPK_doublereal, ldb: *const CLPK_integer, w: CLPK_doublereal,
            work: CLPK_doublereal, lwork: CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
}
