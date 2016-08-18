// Copyright 2015 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.
use std::cmp;
use std::ops::Drop;
use std::ptr;
use matrix::Matrix;
use types::Order;
use types::Order::*;

/// ColMem is a utility structure for temporarily representing data
/// from another matrix as in column-major format.
pub struct ColMem<'a, T: 'a> {
    source: &'a mut Matrix<T>,
    lead: i32,
    data: Option<Vec<T>>,
}

impl<'a, T> ColMem<'a, T> {
    pub fn new<'b>(order: Order, mat: &'b mut Matrix<T>) -> ColMem<'b, T> {
        let (lead, data) = match order {
            ColMajor => (mat.rows(), None),
            RowMajor => {
                let m = mat.rows();
                let n = mat.cols();
                let lead = n;
                let lead_t = cmp::max(1, m);

                let len_t = (lead_t * cmp::max(1, n)) as usize;
                let mut transpose = Vec::with_capacity(len_t);

                unsafe {
                    transpose.set_len(len_t);

                    transpose_data(Order::RowMajor, m as isize, n as isize,
                        mat.as_ptr(), lead as isize,
                        transpose.as_mut_ptr(), lead_t as isize);
                }

                (lead_t, Some(transpose))
            },
        };

        ColMem {
            source: mat,
            lead: lead,
            data: data,
        }
    }

    /// Return the leading storage dimension of the matrix.
    pub fn lead(&self) -> i32 { self.lead }

    /// Return a mutable pointer to the data.
    pub fn as_mut_ptr(&mut self) -> *mut T {
        match self.data {
            Some(ref mut v) => v.as_mut_ptr(),
            None => self.source.as_mut_ptr(),
        }
    }

    fn write_back(&mut self) {
        let transpose = match self.data {
            Some(ref v) => v,
            None => return,
        };

        let ref mut mat: &mut Matrix<T> = self.source;
        let m = mat.rows();
        let n = mat.cols();
        let lead = n;
        let lead_t = cmp::max(1, m);

        unsafe {
            transpose_data(Order::ColMajor, m as isize, n as isize, transpose.as_ptr(), lead_t as isize, mat.as_mut_ptr(), lead as isize);
        }
    }
}

impl<'a, T> Drop for ColMem<'a, T> {
    fn drop(&mut self) {
        self.write_back();
    }
}

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
