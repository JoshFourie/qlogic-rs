use super::matrix::Matrix;
use std::ops::{ AddAssign };

pub struct MatrixIter<M>
{
    matrix: M,
    _iter: usize,
}

impl<T: Copy> IntoIterator for Matrix<T>
{
    type Item = T;
    type IntoIter = MatrixIter<Self>;
    fn into_iter(self) -> Self::IntoIter
    {
        MatrixIter {
            matrix: self,
            _iter: 0,
        }
    }
}

impl<'a, T: Copy> IntoIterator for &'a Matrix<T>
{
    type Item = T;
    type IntoIter = MatrixIter<Self>;
    fn into_iter(self) -> Self::IntoIter{
        MatrixIter {
            matrix: self,
            _iter: 0,
        }
    }
}

impl<'a, T: Copy> Iterator for MatrixIter<&'a Matrix<T>>
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item>
    {
        let i = self._iter;
        match self.matrix.dim? > i 
        {
            true => {
                self._iter.add_assign(1);
                Some(self.matrix.inner[i])
            },
            false => None
        } 
    }
}

impl<T: Copy> Iterator for MatrixIter<Matrix<T>>
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item>
    {
        let i = self._iter;
        match self.matrix.dim? > i 
        {
            true => {
                self._iter.add_assign(1);
                Some(self.matrix.inner[i])
            },
            false => None
        } 
    }
}