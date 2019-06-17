//! A module containing the `MatrixIter` structure for iteration.
//! 
//! The structure currently takes ownership of either a `Matrix` structure,
//! or a copy of a `Matrix` structure but might be extended to store
//! a reference to a `Matrix` structure to avoid copies.

use crate::matrix;

/// A struct for iterating over the `Matrix` structure. Type parameters
/// require a `Copy` bound to copy out of a slice.
pub struct MatrixIter<T> {
    mat: matrix::Matrix<T>,
    count: usize,
}

impl<T:Copy> Iterator for MatrixIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let idx: usize = self.count;
        if self.mat.inner.len() > idx {
            self.count += 1;
            Some(self.mat.inner[idx])
        } else { None }
    }
}

impl<T:Copy> IntoIterator for matrix::Matrix<T>
{
    type Item = T;

    type IntoIter = MatrixIter<T>;

    fn into_iter(self) -> Self::IntoIter
    {
        MatrixIter {
            mat: self,
            count: 0
        }
    }
}

impl<'a, T:Copy> IntoIterator for &'a matrix::Matrix<T>
{
    type Item = T;

    type IntoIter = MatrixIter<T>;

    fn into_iter(self) -> Self::IntoIter
    {
        MatrixIter {
            mat: self.clone(),
            count: 0
        }
    }
}

impl<'a, T:Copy> IntoIterator for &'a mut matrix::Matrix<T>
{
    type Item = T;

    type IntoIter = MatrixIter<T>;

    fn into_iter(self) -> Self::IntoIter
    {
        MatrixIter {
            mat: self.clone(),
            count: 0
        }
    }
}