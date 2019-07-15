use crate::matrix;
use matrix::interface::{Identity, Transpose};
use matrix::ops::householder::HouseholderReflection;

use num::traits::real;

pub struct HouseholderDecomposition<T> {
    mat: matrix::Matrix<T>
}

impl<T> HouseholderDecomposition<T>
where
    T: real::Real
    + From<f32> 
{
    pub fn new(mat: matrix::Matrix<T>) -> Self {
        Self { mat }
    }

    pub fn qr(self) -> (matrix::Matrix<T>,matrix::Matrix<T>) 
    {
        let mut mat: _ = self.mat;

        let A: matrix::Matrix<T> = mat.clone();
        let col: usize = mat.col; 
        let I: matrix::Matrix<T> = (&mat).identity();

        let mut series_of_Q: Vec<matrix::Matrix<T>> = Vec::with_capacity(col);
        let mut series_of_R: Vec<matrix::Matrix<T>> = Vec::with_capacity(col);

        for k in 0..col-1 {
            let P: matrix::Matrix<T> = HouseholderReflection::new(&mat,k).reflector();
            let mut Q: matrix::Matrix<T> = (&P)*(&mat);

            for i in 0..mat.row {
                Q[k][i] = T::zero();
                Q[i][k] = T::zero();
            }
            
            series_of_Q.push((&P).transpose());
            series_of_R.push(P);

            Q[k][k] = T::one();
            mat = Q;
        }

        let R: matrix::Matrix<T> = series_of_R.into_iter()
            .rev()
            .fold(I.clone(),|acc,Rk| acc * Rk) * A;
        
        let Q: matrix::Matrix<T> = series_of_Q.into_iter()
            .fold(I, |acc,Qk| acc * Qk);

        (Q,R) 
    }
}