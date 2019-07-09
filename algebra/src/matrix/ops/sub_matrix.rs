//! Docs: InProgress, view src.

use crate::matrix;

use matrix::interface;

use std::ops;

impl<T:Copy> interface::Minor<ops::Range<usize>> for matrix::Matrix<T>
{
    fn minor(self, row: ops::Range<usize>, col: ops::Range<usize>) -> Self
    {
        let mut buf: Vec<T> = Vec::new();
        let mut new_row: usize = 0;
        let mut new_col: usize = 0;

        for i in row 
        {
            new_row += 1;
            for j in col.clone() 
            {
                new_col += 1;
                buf.push(self[i][j])
            }
        }

        Self {
            inner: buf,
            row: new_row,
            col: new_col
        }
    }
}