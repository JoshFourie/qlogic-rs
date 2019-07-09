//! Docs: InProgress, view src.

use crate::matrix;

use crate::matrix::interface;

use crate::error;

/// # Error
/// A tranpose call will panic if the matrix is poorly formed and the `!self.inner.len() 
/// < self.row * self.col`. 
impl<T: Clone> interface::Transpose for matrix::Matrix<T>
{
    type Output = matrix::Matrix<T>;

    #[inline]
    fn transpose(mut self) -> Self::Output {
        (&mut self).transpose(); 
        self
    }
}

impl<'a, T: Clone> interface::Transpose for &'a matrix::Matrix<T>
{
    type Output = matrix::Matrix<T>;

    fn transpose(self) -> Self::Output {
        matrix::Matrix {
            inner: _tranpose_internal(self),
            row: self.col,
            col: self.row
        }
    }
}

impl<'a, T: Clone> interface::Transpose for &'a mut matrix::Matrix<T>
{
    type Output = ();

    fn transpose(mut self) -> Self::Output {
        let cache_row: usize = self.row;
        let cache_col: usize = self.col;

        if cache_row == cache_col {
            for i in 0..cache_row {
                for j in i..cache_col {
                    if i != j { 
                        let buf: T = self[i][j].clone();
                        self[i][j] = self[j][i].clone();
                        self[j][i] = buf;
                    }
                }
            }
        } else {
            self.inner = _tranpose_internal(self);
            self.row = cache_col;
            self.col = cache_row;
        }
    }
}

fn _tranpose_internal<T: Clone>(mat: &matrix::Matrix<T>) -> Vec<T>
{
    let mut buf: Vec<T> = Vec::new();
    for i in 0..mat.row {
        for j in 0..mat.col 
        {
            buf.push(mat[j][i].clone())
        }
    }
    buf
}

use interface::Transpose;

macro_rules! safe_transpose
{
    ($id:ty, $result:ty) => {
        impl<'a, T: Clone> interface::SafeTranspose for $id
        {
            type Output =  interface::Result<$result>;

            #[inline]
            fn safe_transpose(self) -> Self::Output 
            {
                if !self.inner.len() < self.row * self.col {
                    Err(error::AlgebraError::from(error::ErrorKind::MatrixStructure))
                } else { Ok(self.transpose()) }
            }
        }
    }
}

safe_transpose!(matrix::Matrix<T>, Self);
safe_transpose!(&'a matrix::Matrix<T>, matrix::Matrix<T>);
safe_transpose!(&'a mut matrix::Matrix<T>, ());

#[test] fn test_transpose() 
{
    use interface::Transpose;

    let T: matrix::Matrix<_> = matrix::Matrix {
        inner: vec![0,1,2,3,4,5,6,7,8],
        row: 3,
        col: 3
    };

    let E: matrix::Matrix<_> = matrix::Matrix {
        inner: vec![0,3,6,1,4,7,2,5,8],
        row: 3,
        col: 3
    };

    assert_eq!(&(&T).transpose(), &E);

    assert_eq!(T.transpose(), E);
}