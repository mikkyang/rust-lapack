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
    pub fn ssyev_(jobz: c_char, uplo: c_char, n: *const CLPK_integer, a: *mut CLPK_real,
            lda: *const CLPK_integer, w: CLPK_real, work: CLPK_real,
            lwork: CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn dsyev_(jobz: c_char, uplo: c_char, n: *const CLPK_integer,
            a: *mut CLPK_doublereal, lda: *const CLPK_integer, w: CLPK_doublereal,
            work: CLPK_doublereal, lwork: CLPK_integer,
            info: *mut CLPK_integer) -> c_int;

    pub fn cheev_(jobz: c_char, uplo: c_char, n: *const CLPK_integer, a: *mut CLPK_complex,
            lda: *const CLPK_integer, w: CLPK_real, work: CLPK_complex,
            lwork: CLPK_integer, rwork: CLPK_real,
            info: *mut CLPK_integer) -> c_int;
    pub fn zheev_(jobz: c_char, uplo: c_char, n: *const CLPK_integer,
            a: *mut CLPK_doublecomplex, lda: *const CLPK_integer,
            w: CLPK_doublereal, work: CLPK_doublecomplex,
            lwork: CLPK_integer, rwork: CLPK_doublereal,
            info: *mut CLPK_integer) -> c_int;

    pub fn sspev_(jobz: c_char, uplo: c_char, n: *const CLPK_integer, ap: CLPK_real,
            w: CLPK_real, z: CLPK_real, ldz: CLPK_integer,
            work: CLPK_real,
            info: *mut CLPK_integer) -> c_int;
    pub fn dspev_(jobz: c_char, uplo: c_char, n: *const CLPK_integer,
            ap: CLPK_doublereal, w: CLPK_doublereal,
            z: CLPK_doublereal, ldz: CLPK_integer,
            work: CLPK_doublereal,
            info: *mut CLPK_integer) -> c_int;

    pub fn chpev_(jobz: c_char, uplo: c_char, n: *const CLPK_integer,
            ap: CLPK_complex, w: CLPK_real, z: CLPK_complex,
            ldz: CLPK_integer, work: CLPK_complex, rwork: CLPK_real,
            info: *mut CLPK_integer) -> c_int;
    pub fn zhpev_(jobz: c_char, uplo: c_char, n: *const CLPK_integer,
            ap: CLPK_doublecomplex, w: CLPK_doublereal,
            z: CLPK_doublecomplex, ldz: CLPK_integer,
            work: CLPK_doublecomplex, rwork: CLPK_doublereal,
            info: *mut CLPK_integer) -> c_int;

    pub fn ssbev_(jobz: c_char, uplo: c_char, n: *const CLPK_integer,
            kd: CLPK_integer, ab: CLPK_real, ldab: CLPK_integer,
            w: CLPK_real, z: CLPK_real, ldz: CLPK_integer,
            work: CLPK_real,
            info: *mut CLPK_integer) -> c_int;
    pub fn dsbev_(jobz: c_char, uplo: c_char, n: *const CLPK_integer,
            kd: CLPK_integer, ab: CLPK_doublereal, ldab: CLPK_integer,
            w: CLPK_doublereal, z: CLPK_doublereal, ldz: CLPK_integer,
            work: CLPK_doublereal,
            info: *mut CLPK_integer) -> c_int;

    pub fn chbev_(jobz: c_char, uplo: c_char, n: *const CLPK_integer,
            kd: CLPK_integer, ab: CLPK_complex, ldab: CLPK_integer,
            w: CLPK_real, z: CLPK_complex, ldz: CLPK_integer,
            work: CLPK_complex, rwork: CLPK_real,
            info: *mut CLPK_integer) -> c_int;
    pub fn zhbev_(jobz: c_char, uplo: c_char, n: *const CLPK_integer,
            kd: CLPK_integer, ab: CLPK_doublecomplex,
            ldab: CLPK_integer, w: CLPK_doublereal,
            z: CLPK_doublecomplex, ldz: CLPK_integer,
            work: CLPK_doublecomplex, rwork: CLPK_doublereal,
            info: *mut CLPK_integer) -> c_int;

    pub fn sstev_(jobz: c_char, n: *const CLPK_integer, d: CLPK_real,
            e: CLPK_real, z: CLPK_real, ldz: CLPK_integer,
            work: CLPK_real,
            info: *mut CLPK_integer) -> c_int;
    pub fn dstev_(jobz: c_char, n: *const CLPK_integer, d: CLPK_doublereal,
            e: CLPK_doublereal, z: CLPK_doublereal, ldz: CLPK_integer,
            work: CLPK_doublereal,
            info: *mut CLPK_integer) -> c_int;

    pub fn sgeev_(jobvl: c_char, jobvr: c_char, n: *const CLPK_integer, a: *mut CLPK_real,
            lda: *const CLPK_integer, wr: CLPK_real, wi: CLPK_real,
            vl: CLPK_real, ldvl: CLPK_integer, vr: CLPK_real,
            ldvr: CLPK_integer, work: CLPK_real, lwork: CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn dgeev_(jobvl: c_char, jobvr: c_char, n: *const CLPK_integer,
            a: *mut CLPK_doublereal, lda: *const CLPK_integer, wr: CLPK_doublereal,
            wi: CLPK_doublereal, vl: CLPK_doublereal,
            ldvl: CLPK_integer, vr: CLPK_doublereal, ldvr: CLPK_integer,
            work: CLPK_doublereal, lwork: CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn cgeev_(jobvl: c_char, jobvr: c_char, n: *const CLPK_integer,
            a: *mut CLPK_complex, lda: *const CLPK_integer, w: CLPK_complex,
            vl: CLPK_complex, ldvl: CLPK_integer, vr: CLPK_complex,
            ldvr: CLPK_integer, work: CLPK_complex, lwork: CLPK_integer,
            rwork: CLPK_real,
            info: *mut CLPK_integer) -> c_int;
    pub fn zgeev_(jobvl: c_char, jobvr: c_char, n: *const CLPK_integer,
            a: *mut CLPK_doublecomplex, lda: *const CLPK_integer,
            w: CLPK_doublecomplex, vl: CLPK_doublecomplex,
            ldvl: CLPK_integer, vr: CLPK_doublecomplex,
            ldvr: CLPK_integer, work: CLPK_doublecomplex,
            lwork: CLPK_integer, rwork: CLPK_doublereal,
            info: *mut CLPK_integer) -> c_int;
}
