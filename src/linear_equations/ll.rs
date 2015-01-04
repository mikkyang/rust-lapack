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

    pub fn sgtsv_(n: *const CLPK_integer, nrhs: *const CLPK_integer, dl: *mut CLPK_real,
            d: *mut CLPK_real, du: *mut CLPK_real, b: *mut CLPK_real,
            ldb: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn dgtsv_(n: *const CLPK_integer, nrhs: *const CLPK_integer, dl: *mut CLPK_doublereal,
            d: *mut CLPK_doublereal, du: *mut CLPK_doublereal,
            b: *mut CLPK_doublereal, ldb: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn cgtsv_(n: *const CLPK_integer, nrhs: *const CLPK_integer, dl: *mut CLPK_complex,
            d: *mut CLPK_complex, du: *mut CLPK_complex, b: *mut CLPK_complex,
            ldb: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn zgtsv_(n: *const CLPK_integer, nrhs: *const CLPK_integer,
            dl: *mut CLPK_doublecomplex, d: *mut CLPK_doublecomplex,
            du: *mut CLPK_doublecomplex, b: *mut CLPK_doublecomplex,
            ldb: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;

    pub fn sposv_(uplo: *const c_char, n: *const CLPK_integer, nrhs: *const CLPK_integer,
            a: *mut CLPK_real, lda: *const CLPK_integer, b: *mut CLPK_real,
            ldb: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn dposv_(uplo: *const c_char, n: *const CLPK_integer, nrhs: *const CLPK_integer,
            a: *mut CLPK_doublereal, lda: *const CLPK_integer, b: *mut CLPK_doublereal,
            ldb: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn cposv_(uplo: *const c_char, n: *const CLPK_integer, nrhs: *const CLPK_integer,
            a: *mut CLPK_complex, lda: *const CLPK_integer, b: *mut CLPK_complex,
            ldb: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn zposv_(uplo: *const c_char, n: *const CLPK_integer, nrhs: *const CLPK_integer,
            a: *mut CLPK_doublecomplex, lda: *const CLPK_integer,
            b: *mut CLPK_doublecomplex, ldb: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;

    pub fn sppsv_(uplo: *const c_char, n: *const CLPK_integer, nrhs: *const CLPK_integer,
            ap: *mut CLPK_real, b: *mut CLPK_real, ldb: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn dppsv_(uplo: *const c_char, n: *const CLPK_integer, nrhs: *const CLPK_integer,
            ap: *mut CLPK_doublereal, b: *mut CLPK_doublereal, ldb: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn cppsv_(uplo: *const c_char, n: *const CLPK_integer, nrhs: *const CLPK_integer,
            ap: *mut CLPK_complex, b: *mut CLPK_complex, ldb: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn zppsv_(uplo: *const c_char, n: *const CLPK_integer, nrhs: *const CLPK_integer,
            ap: *mut CLPK_doublecomplex, b: *mut CLPK_doublecomplex,
            ldb: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;

    pub fn spbsv_(uplo: *const c_char, n: *const CLPK_integer, kd: *const CLPK_integer,
            nrhs: *const CLPK_integer, ab: *mut CLPK_real, ldab: *const CLPK_integer,
            b: *mut CLPK_real, ldb: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn dpbsv_(uplo: *const c_char, n: *const CLPK_integer, kd: *const CLPK_integer,
            nrhs: *const CLPK_integer, ab: *mut CLPK_doublereal, ldab: *const CLPK_integer,
            b: *mut CLPK_doublereal, ldb: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn cpbsv_(uplo: *const c_char, n: *const CLPK_integer, kd: *const CLPK_integer,
            nrhs: *const CLPK_integer, ab: *mut CLPK_complex, ldab: *const CLPK_integer,
            b: *mut CLPK_complex, ldb: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn zpbsv_(uplo: *const c_char, n: *const CLPK_integer, kd: *const CLPK_integer,
            nrhs: *const CLPK_integer, ab: *mut CLPK_doublecomplex,
            ldab: *const CLPK_integer, b: *mut CLPK_doublecomplex,
            ldb: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;

    pub fn sptsv_(n: *const CLPK_integer, nrhs: *const CLPK_integer, d: *mut CLPK_real,
            e: *mut CLPK_real, b: *mut CLPK_real, ldb: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn dptsv_(n: *const CLPK_integer, nrhs: *const CLPK_integer,
            d: *mut CLPK_doublereal, e: *mut CLPK_doublereal,
            b: *mut CLPK_doublereal, ldb: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn cptsv_(n: *const CLPK_integer, nrhs: *const CLPK_integer, d: *mut CLPK_real,
            e: *mut CLPK_complex, b: *mut CLPK_complex, ldb: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn zptsv_(n: *const CLPK_integer, nrhs: *const CLPK_integer,
            d: *mut CLPK_doublereal, e: *mut CLPK_doublecomplex,
            b: *mut CLPK_doublecomplex, ldb: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;

    pub fn ssysv_(uplo: *const c_char, n: *const CLPK_integer, nrhs: *const CLPK_integer,
            a: *mut CLPK_real, lda: *const CLPK_integer, ipiv: *mut CLPK_integer,
            b: *mut CLPK_real, ldb: *const CLPK_integer, work: *mut CLPK_real,
            lwork: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn dsysv_(uplo: *const c_char, n: *const CLPK_integer, nrhs: *const CLPK_integer,
            a: *mut CLPK_doublereal, lda: *const CLPK_integer, ipiv: *mut CLPK_integer,
            b: *mut CLPK_doublereal, ldb: *const CLPK_integer,
            work: *mut CLPK_doublereal, lwork: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn csysv_(uplo: *const c_char, n: *const CLPK_integer, nrhs: *const CLPK_integer,
            a: *mut CLPK_complex, lda: *const CLPK_integer, ipiv: *mut CLPK_integer,
            b: *mut CLPK_complex, ldb: *const CLPK_integer, work: *mut CLPK_complex,
            lwork: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn zsysv_(uplo: *const c_char, n: *const CLPK_integer, nrhs: *const CLPK_integer,
            a: *mut CLPK_doublecomplex, lda: *const CLPK_integer,
            ipiv: *mut CLPK_integer, b: *mut CLPK_doublecomplex,
            ldb: *const CLPK_integer, work: *mut CLPK_doublecomplex,
            lwork: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;

    pub fn chesv_(uplo: *const c_char, n: *const CLPK_integer, nrhs: *const CLPK_integer,
            a: *mut CLPK_complex, lda: *const CLPK_integer, ipiv: *mut CLPK_integer,
            b: *mut CLPK_complex, ldb: *const CLPK_integer, work: *mut CLPK_complex,
            lwork: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn zhesv_(uplo: *const c_char, n: *const CLPK_integer, nrhs: *const CLPK_integer,
            a: *mut CLPK_doublecomplex, lda: *const CLPK_integer,
            ipiv: *mut CLPK_integer, b: *mut CLPK_doublecomplex,
            ldb: *const CLPK_integer, work: *mut CLPK_doublecomplex,
            lwork: *const CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
}
