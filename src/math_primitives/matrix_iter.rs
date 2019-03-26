use super::matrix::Matrix;
use super::matrix_wrap::{ SquareMatrix, Vector };
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

impl<T: Copy> IntoIterator for SquareMatrix<T>
{
    type Item = T;
    type IntoIter = MatrixIter<Matrix<T>>;
    fn into_iter(self) -> Self::IntoIter
    {
        Matrix::from(self).into_iter()
    }
}

impl <'a, T: Copy> IntoIterator for &'a SquareMatrix<T>
{
    type Item = T;
    type IntoIter = MatrixIter<Matrix<T>>;
    fn into_iter(self) -> Self::IntoIter
    {
        Matrix::from(self).into_iter()
    }
}

impl<T: Copy> IntoIterator for Vector<T>
{
    type Item = T;
    type IntoIter = MatrixIter<Matrix<T>>;
    fn into_iter(self) -> Self::IntoIter
    {
        Matrix::from(self).into_iter()
    }
}

impl <'a, T: Copy> IntoIterator for &'a Vector<T>
{
    type Item = T;
    type IntoIter = MatrixIter<Matrix<T>>;
    fn into_iter(self) -> Self::IntoIter
    {
        Matrix::from(self).into_iter()
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