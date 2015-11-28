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

extern "C" {
    pub fn sgels_(trans: *const c_char, m: *const CLPK_integer, n: *const CLPK_integer,
            nrhs: *const CLPK_integer, a: *mut CLPK_real, lda: *const CLPK_integer,
            b: *mut CLPK_real, ldb: *const CLPK_integer, work: *mut CLPK_real,
            lwork: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn dgels_(trans: *const c_char, m: *const CLPK_integer, n: *const CLPK_integer,
            nrhs: *const CLPK_integer, a: *mut CLPK_doublereal, lda: *const CLPK_integer,
            b: *mut CLPK_doublereal, ldb: *const CLPK_integer,
            work: *mut CLPK_doublereal, lwork: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn cgels_(trans: *const c_char, m: *const CLPK_integer, n: *const CLPK_integer,
            nrhs: *const CLPK_integer, a: *mut CLPK_complex, lda: *const CLPK_integer,
            b: *mut CLPK_complex, ldb: *const CLPK_integer, work: *mut CLPK_complex,
            lwork: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn zgels_(trans: *const c_char, m: *const CLPK_integer, n: *const CLPK_integer,
            nrhs: *const CLPK_integer, a: *mut CLPK_doublecomplex,
            lda: *const CLPK_integer, b: *mut CLPK_doublecomplex, ldb: *const CLPK_integer,
            work: *mut CLPK_doublecomplex, lwork: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;

    pub fn sgglse_(m: CLPK_integer, n: *const CLPK_integer, p: CLPK_integer,
            a: *mut CLPK_real, lda: *const CLPK_integer, b: *mut CLPK_real,
            ldb: *const CLPK_integer, c: CLPK_real, d: CLPK_real,
            x: CLPK_real, work: CLPK_real, lwork: CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn dgglse_(m: CLPK_integer, n: *const CLPK_integer, p: CLPK_integer,
            a: *mut CLPK_doublereal, lda: *const CLPK_integer, b: *mut CLPK_doublereal,
            ldb: *const CLPK_integer, c: CLPK_doublereal,
            d: CLPK_doublereal, x: CLPK_doublereal,
            work: CLPK_doublereal, lwork: CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn cgglse_(m: CLPK_integer, n: *const CLPK_integer, p: CLPK_integer,
            a: *mut CLPK_complex, lda: *const CLPK_integer, b: *mut CLPK_complex,
            ldb: *const CLPK_integer, c: CLPK_complex, d: CLPK_complex,
            x: CLPK_complex, work: CLPK_complex, lwork: CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn zgglse_(m: CLPK_integer, n: *const CLPK_integer, p: CLPK_integer,
            a: *mut CLPK_doublecomplex, lda: *const CLPK_integer,
            b: *mut CLPK_doublecomplex, ldb: *const CLPK_integer,
            c: CLPK_doublecomplex, d: CLPK_doublecomplex,
            x: CLPK_doublecomplex, work: CLPK_doublecomplex,
            lwork: CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
}
