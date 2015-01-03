// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

macro_rules! prefix(
    (f32, $f: ident) => (concat_idents!(s, $f));
    (f64, $f: ident) => (concat_idents!(d, $f));
    (Complex32, $f: ident) => (concat_idents!(c, $f));
    (Complex64, $f: ident) => (concat_idents!(z, $f));
);
