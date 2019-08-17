use std::ops;

use crate::matrix;
use matrix::interface;
use matrix::ops::submatrix;

macro_rules! impl_minor {
    ($id:ty) => {
        impl<'a, T:Copy> interface::Minor<(ops::Range<usize>,ops::Range<usize>)> for $id
        {
            type Output = matrix::Matrix<T>;

            fn minor(self, range: (ops::Range<usize>,ops::Range<usize>)) -> Self::Output
            {
                let (row,col): _ = range;
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

        impl<'a, T:Copy> interface::Minor<submatrix::SubMatrixRange> for $id
        {
            type Output = matrix::Matrix<T>;

            fn minor(self, range: submatrix::SubMatrixRange) -> Self::Output
            {
                let (row,col): _ = range.into_tuple();
                let mut buf: Vec<T> = Vec::new();
                let new_row: usize = row.end -row.start;
                let new_col: usize = col.end -col.start;

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
    use matrix::ops::submatrix;

    #[test] fn test_minor()
    {
        let A: matrix::Matrix<usize> = vec![
            0,0,0,0,0,
            0,1,1,1,0,
            0,1,1,1,0,
            0,1,1,1,0,
            0,0,0,0,0,
        ].into();

        let test: _ = A.minor(submatrix::SubMatrixRange::new(1..3, 1..4));
        let exp: matrix::Matrix<usize> = matrix::Matrix::new(vec![
            1,1,1,
            1,1,1,
        ],2,3);

        assert_eq!(test,exp);
    }
}