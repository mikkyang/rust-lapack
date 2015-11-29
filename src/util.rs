// Copyright 2015 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.
use std::cmp;
use types::Layout;
use types::Layout::*;

pub fn transpose_data<T: Sized + Clone>(initial_layout: Layout, m: usize, n: usize, input: &[T], ld_input: usize, output: &mut [T], ld_output: usize) {
    let (x, y) = match initial_layout {
        ColMajor => (n, m),
        RowMajor => (m, n),
    };

    for i in 0..cmp::min(y, ld_input) {
        for j in 0..cmp::min(x, ld_output) {
            output[i * ld_output + j] = input[j * ld_input + i].clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem;
    use types::Layout::*;
    use util::transpose_data;

    #[test]
    fn transpose() {
        let i: [f64; 6] = [2.0,4.0,7.0,3.0,9.0,4.0];
        let mut buf: [f64; 6] = unsafe {mem::zeroed()};
        let o: [f64; 6] = [2.0,3.0,4.0,9.0,7.0,4.0];

        transpose_data(ColMajor, 3, 2, &i, 3, &mut buf, 2);
        assert_eq!(buf, o);
    }
}
