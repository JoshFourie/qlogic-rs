//! Docs: InProgress, view src.

use crate::vector;

use crate::matrix;

use matrix::interface::{Column, Row};

use vector::interface::Dot;

use std::ops;

impl<T:Copy> ops::Mul<vector::Vector<T>> for matrix::Matrix<T>
where
    T: ops::Mul<Output=T>
    + num::Zero
{
    type Output = vector::Vector<T>;

    fn mul(self, rhs: vector::Vector<T>) -> Self::Output
    {
        let mut new: vector::Vector<T> = Default::default();

        for i in 0..self.row {
            let vector: vector::Vector<T> = (&self).get_row(i).into();
            new.push(rhs.clone().dot(vector))
        }

        new
    }
}

impl<T:Copy> ops::Mul<matrix::Matrix<T>> for vector::Vector<T>
where
    T: ops::Mul<Output=T>
    + num::Zero
{
    type Output = vector::Vector<T>;

    fn mul(self, rhs: matrix::Matrix<T>) -> Self::Output
    {  
        let mut new: vector::Vector<T> = Default::default();

        for i in 0..rhs.col {
            let vector: vector::Vector<T> = (&rhs).get_col(i).into();
            new.push(vector.clone().dot(vector))
        }

        new
    }
}

#[cfg(test)] mod tests 
{
    use crate::vector;

    use crate::matrix;

    #[test] fn test_matrix_vector_product()
    {
        let matrix: matrix::Matrix<_> = matrix::Matrix {
            inner: vec![1, -1, 2, 0, -3, 1],
            row: 2,
            col: 3
        };

        let vector: vector::Vector<_> = vec![2, 1, 0].into();

        let exp: vector::Vector<_> = vec![1, -3].into();

        assert_eq!(matrix * vector, exp);
    }

    #[test] fn test_vector_matrix_product()
    {
        let matrix: matrix::Matrix<_> = matrix::Matrix {
            inner: vec![2, 1, 1, 1, 1, 1, 1, 1, 1],
            row: 3,
            col: 3
        };

        let vector: vector::Vector<_> = vec![1, 1, 1].into();

        let exp: vector::Vector<_> = vec![4, 3, 3].into();

        assert_eq!(matrix * vector, exp);
    }
} 