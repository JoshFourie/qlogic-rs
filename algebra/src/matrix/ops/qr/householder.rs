use crate::matrix;
use matrix::interface::{Identity, Transpose};
use matrix::ops::householder;

use num::traits::real;

use std::ops;

pub struct HouseholderDecomposition<T> {
    Q: matrix::Matrix<T>,
    R: matrix::Matrix<T>
}

impl<T> HouseholderDecomposition<T>
where
    T: real::Real + From<f32> 
{
    pub fn new(mut mat: matrix::Matrix<T>) -> Self {

        let buf: matrix::Matrix<T> = mat.clone();
        let mut series: _ = HouseholderSeries::new(&buf);

        for k in 0..mat.col-1 {
            let (Q,R): _ = HouseholderDecomposition::single_round(&mut mat, k).into_tuple();
            series.push(Q,R);   
        }
        series.build()
    }

    fn single_round(mut mat: &mut matrix::Matrix<T>, k: usize) -> Self {
        let mut P: matrix::Matrix<T> = householder::Householder::new(&mat,k).into();

        *mat = &mut P * mat.clone();
        for i in 0..mat.row {
            mat[k][i] = T::zero();
            mat[i][k] = T::zero();
        }
        mat[k][k] = T::one();
        
        Self {
            Q: (&P).transpose(),
            R: P
        }
    }

    pub fn into_tuple(self) -> (matrix::Matrix<T>, matrix::Matrix<T>) {
        (self.Q, self.R)
    }
}

struct HouseholderSeries<'a,T> {
    Q: Vec<matrix::Matrix<T>>,
    R: Vec<matrix::Matrix<T>>,
    buf: &'a matrix::Matrix<T>
}

impl<'a,T: Copy> HouseholderSeries<'a,T> 
where   
    T: num::One 
    + num::Zero
    + ops::Mul<T,Output=T>
{
    fn new(buf: &'a matrix::Matrix<T>) -> Self {
        Self {
            Q: Vec::new(),
            R: Vec::new(),
            buf
        }
    }

    fn push(&mut self, Q: matrix::Matrix<T>, R: matrix::Matrix<T>) {
        self.Q.push(Q);
        self.R.push(R);
    }

    fn build(self) -> HouseholderDecomposition<T> {
        let I: matrix::Matrix<T> = self.buf.identity();

        let R: matrix::Matrix<T> = self.R
            .into_iter()
            .rev()
            .fold(I.clone(),|acc,Rk| acc * Rk) * self.buf;
        let Q: matrix::Matrix<T> = self.Q
            .into_iter()
            .fold(I, |acc,Qk| acc * Qk);

        HouseholderDecomposition { Q,R }
    }
}
