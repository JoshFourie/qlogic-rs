//! Docs: InProgress, view src.

use crate::matrix;

use matrix::interface;

use std::ops;

macro_rules! impl_minor {
    ($id:ty) => {
        impl<'a, T:Copy> interface::Minor<ops::Range<usize>> for $id
        {
            type Output = matrix::Matrix<T>;

            fn minor(self, row: ops::Range<usize>, col: ops::Range<usize>) -> Self::Output
            {
                let mut buf: Vec<T> = Vec::new();
                let new_row: usize = row.end-row.start;
                let new_col: usize = col.end-col.start;

                for i in row {
                    for j in col.clone() {
                        buf.push(self[i][j])
                    }
                }

                matrix::Matrix {
                    inner: buf,
                    row: new_row,
                    col: new_col
                }
            }
        }
    }
}

impl_minor!(matrix::Matrix<T>);
impl_minor!(&'a matrix::Matrix<T>);
impl_minor!(&'a mut matrix::Matrix<T>);

#[cfg(test)]
mod tests
{
    use crate::matrix;
    use matrix::interface::Minor;

    #[test] fn test_minor()
    {
        let A: matrix::Matrix<usize> = vec![
            0,0,0,0,0,
            0,1,1,1,0,
            0,1,1,1,0,
            0,1,1,1,0,
            0,0,0,0,0,
        ].into();

        let test: _ = A.minor(1..3, 1..4);
        let exp: matrix::Matrix<usize> = matrix::Matrix::new(vec![
            1,1,1,
            1,1,1,
        ],2,3);

        assert_eq!(test,exp);
    }
}