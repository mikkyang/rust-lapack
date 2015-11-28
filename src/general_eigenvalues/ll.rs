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

    pub fn chegv_(itype: CLPK_integer, jobz: c_char, uplo: c_char,
            n: *const CLPK_integer, a: *mut CLPK_complex, lda: *const CLPK_integer,
            b: *mut CLPK_complex, ldb: *const CLPK_integer, w: CLPK_real,
            work: CLPK_complex, lwork: CLPK_integer, rwork: CLPK_real,
            info: *mut CLPK_integer) -> c_int;
    pub fn zhegv_(itype: CLPK_integer, jobz: c_char, uplo: c_char,
            n: *const CLPK_integer, a: *mut CLPK_doublecomplex, lda: *const CLPK_integer,
            b: *mut CLPK_doublecomplex, ldb: *const CLPK_integer,
            w: CLPK_doublereal, work: CLPK_doublecomplex,
            lwork: CLPK_integer, rwork: CLPK_doublereal,
            info: *mut CLPK_integer) -> c_int;

    pub fn sspgv_(itype: CLPK_integer, jobz: c_char, uplo: c_char,
            n: *const CLPK_integer, ap: CLPK_real, bp: CLPK_real,
            w: CLPK_real, z: CLPK_real, ldz: CLPK_integer,
            work: CLPK_real,
            info: *mut CLPK_integer) -> c_int;
    pub fn dspgv_(itype: CLPK_integer, jobz: c_char, uplo: c_char,
            n: *const CLPK_integer, ap: CLPK_doublereal, bp: CLPK_doublereal,
            w: CLPK_doublereal, z: CLPK_doublereal, ldz: CLPK_integer,
            work: CLPK_doublereal,
            info: *mut CLPK_integer) -> c_int;

    pub fn chpgv_(itype: CLPK_integer, jobz: c_char, uplo: c_char,
            n: *const CLPK_integer, ap: CLPK_complex, bp: CLPK_complex,
            w: CLPK_real, z: CLPK_complex, ldz: CLPK_integer,
            work: CLPK_complex, rwork: CLPK_real,
            info: *mut CLPK_integer) -> c_int;
    pub fn zhpgv_(itype: CLPK_integer, jobz: c_char, uplo: c_char,
            n: *const CLPK_integer, ap: CLPK_doublecomplex,
            bp: CLPK_doublecomplex, w: CLPK_doublereal,
            z: CLPK_doublecomplex, ldz: CLPK_integer,
            work: CLPK_doublecomplex, rwork: CLPK_doublereal,
            info: *mut CLPK_integer) -> c_int;

    pub fn ssbgv_(jobz: c_char, uplo: c_char, n: *const CLPK_integer,
            ka: *mut CLPK_integer, kb: CLPK_integer, ab: CLPK_real,
            ldab: CLPK_integer, bb: CLPK_real, ldbb: CLPK_integer,
            w: CLPK_real, z: CLPK_real, ldz: CLPK_integer,
            work: CLPK_real,
            info: *mut CLPK_integer) -> c_int;
    pub fn dsbgv_(jobz: c_char, uplo: c_char, n: *const CLPK_integer,
            ka: *mut CLPK_integer, kb: CLPK_integer, ab: CLPK_doublereal,
            ldab: CLPK_integer, bb: CLPK_doublereal, ldbb: CLPK_integer,
            w: CLPK_doublereal, z: CLPK_doublereal, ldz: CLPK_integer,
            work: CLPK_doublereal,
            info: *mut CLPK_integer) -> c_int;

    pub fn chbgv_(jobz: c_char, uplo: c_char, n: *const CLPK_integer,
            ka: *mut CLPK_integer, kb: CLPK_integer, ab: CLPK_complex,
            ldab: CLPK_integer, bb: CLPK_complex, ldbb: CLPK_integer,
            w: CLPK_real, z: CLPK_complex, ldz: CLPK_integer,
            work: CLPK_complex, rwork: CLPK_real,
            info: *mut CLPK_integer) -> c_int;
    pub fn zhbgv_(jobz: c_char, uplo: c_char, n: *const CLPK_integer,
            ka: *mut CLPK_integer, kb: CLPK_integer, ab: CLPK_doublecomplex,
            ldab: CLPK_integer, bb: CLPK_doublecomplex,
            ldbb: CLPK_integer, w: CLPK_doublereal,
            z: CLPK_doublecomplex, ldz: CLPK_integer,
            work: CLPK_doublecomplex, rwork: CLPK_doublereal,
            info: *mut CLPK_integer) -> c_int;

    pub fn sggev_(jobvl: c_char, jobvr: c_char, n: *const CLPK_integer, a: *mut CLPK_real,
            lda: *const CLPK_integer, b: *mut CLPK_real, ldb: *const CLPK_integer,
            alphar: CLPK_real, alphai: CLPK_real, beta: *mut CLPK_real,
            vl: CLPK_real, ldvl: CLPK_integer, vr: CLPK_real,
            ldvr: CLPK_integer, work: CLPK_real, lwork: CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn dggev_(jobvl: c_char, jobvr: c_char, n: *const CLPK_integer,
            a: *mut CLPK_doublereal, lda: *const CLPK_integer, b: *mut CLPK_doublereal,
            ldb: *const CLPK_integer, alphar: CLPK_doublereal,
            alphai: CLPK_doublereal, beta: *mut CLPK_doublereal,
            vl: CLPK_doublereal, ldvl: CLPK_integer,
            vr: CLPK_doublereal, ldvr: CLPK_integer,
            work: CLPK_doublereal, lwork: CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn cggev_(jobvl: c_char, jobvr: c_char, n: *const CLPK_integer,
            a: *mut CLPK_complex, lda: *const CLPK_integer, b: *mut CLPK_complex,
            ldb: *const CLPK_integer, alpha: *mut CLPK_complex, beta: *mut CLPK_complex,
            vl: CLPK_complex, ldvl: CLPK_integer, vr: CLPK_complex,
            ldvr: CLPK_integer, work: CLPK_complex, lwork: CLPK_integer,
            rwork: CLPK_real,
            info: *mut CLPK_integer) -> c_int;
    pub fn zggev_(jobvl: c_char, jobvr: c_char, n: *const CLPK_integer,
            a: *mut CLPK_doublecomplex, lda: *const CLPK_integer,
            b: *mut CLPK_doublecomplex, ldb: *const CLPK_integer,
            alpha: *mut CLPK_doublecomplex, beta: *mut CLPK_doublecomplex,
            vl: CLPK_doublecomplex, ldvl: CLPK_integer,
            vr: CLPK_doublecomplex, ldvr: CLPK_integer,
            work: CLPK_doublecomplex, lwork: CLPK_integer,
            rwork: CLPK_doublereal,
            info: *mut CLPK_integer) -> c_int;
}
