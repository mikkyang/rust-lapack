// Copyright 2015 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.
use std::cmp;
use std::ptr;
use types::Order;
use types::Order::*;

pub unsafe fn transpose_data<T>(initial_layout: Order, m: isize, n: isize, input: *const T, ld_input: isize, output: *mut T, ld_output: isize) {
    let (x, y) = match initial_layout {
        ColMajor => (n, m),
        RowMajor => (m, n),
    };

    for i in 0..cmp::min(y, ld_input) {
        for j in 0..cmp::min(x, ld_output) {
            ptr::write(
                output.offset(i * ld_output + j),
            ptr::read(
                input.offset(j * ld_input + i)));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem;
    use types::Order::*;
    use util::transpose_data;

    #[test]
    fn transpose() {
        let i: [f64; 6] = [2.0,4.0,7.0,3.0,9.0,4.0];
        let mut buf: [f64; 6] = unsafe {mem::zeroed()};
        let o: [f64; 6] = [2.0,3.0,4.0,9.0,7.0,4.0];

        unsafe {
            transpose_data(ColMajor, 3, 2, (&i).as_ptr(), 3, (&mut buf).as_mut_ptr(), 2);
        }
        assert_eq!(buf, o);
    }
}
