// Copyright 2015 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

#[derive(Debug)]
pub enum Error {
    DimensionMismatch,
    IllegalParameter(usize),
    DiagonalElementZero(usize),
}
