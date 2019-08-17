use crate::matrix;
use matrix::interface::{Identity, Minor, SubMatrix, Transpose};
use matrix::ops::{householder,submatrix};

use num::traits::real;

use std::ops;

pub struct HouseholderDecomposition<T> {
    Q: matrix::Matrix<T>,
    A: matrix::Matrix<T>,
    R: matrix::Matrix<T>,
}

impl<T> HouseholderDecomposition<T>
where
    T: real::Real + From<f32> 
{
    pub fn new(mut mat: matrix::Matrix<T>) -> Self {
        let mut series: _ = HouseholderSeries::prepare_new(mat.clone());
        for k in 0..mat.col-1 {
            let (Q,R): _ = HouseholderDecomposition::single_round(&mat, k);
            mat = &Q * mat;
            series.push(Q,R);
            
        }
        series.build()
    }

    pub fn single_round(mat: &matrix::Matrix<T>, k: usize) -> (matrix::Matrix<T>, matrix::Matrix<T>) {
        let P: matrix::Matrix<T> = {
            let range: _ = submatrix::SubMatrixRange::new(k..mat.row, k..mat.col);
            let minor: _ = mat.minor(range.clone());
            let house: _ = householder::Householder::new(&minor, 0).into();
            let out: _ = mat.identity().insert_minor(house, range);
            out
        };

        let Q: _ = (&P).transpose();
        let R: _ = P;
        (Q,R)
    }

    pub fn into_tuple(self) -> (matrix::Matrix<T>, matrix::Matrix<T>, matrix::Matrix<T>) {
        (self.Q, self.A, self.R)
    }
}

struct HouseholderSeries<T> {
    Q: Vec<matrix::Matrix<T>>,
    A: matrix::Matrix<T>,
    R: Vec<matrix::Matrix<T>>
}

impl<T: Copy> HouseholderSeries<T> 
where   
    T: num::One 
    + num::Zero
    + ops::Mul<T,Output=T>
{
    fn prepare_new(A: matrix::Matrix<T>) -> Self {
        Self {
            Q: Vec::new(),
            R: Vec::new(),
            A
        }
    }

    fn push(&mut self, Q: matrix::Matrix<T>, R: matrix::Matrix<T>) {
        self.Q.push(Q);
        self.R.push(R);
    }

    fn build(self) -> HouseholderDecomposition<T> {
        let I: matrix::Matrix<T> = (&self.A).identity();
        let R: matrix::Matrix<T> = self.R
            .into_iter()
            .rev()
            .fold(I.clone(),|acc, Rk| acc * Rk) * &self.A;
        let Q: matrix::Matrix<T> = self.Q
            .into_iter()
            .fold(I, |acc,Qk| acc * Qk);

        HouseholderDecomposition { 
            Q,
            R,
            A: self.A 
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::ApproxEq;

    #[test]
    fn test_series_build() {
        let A: matrix::Matrix<f64> = vec![
            12.0, -51.0, 4.0, 
            6.0, 167.0, -68.0, 
            -4.0, 24.0, -41.0
        ].into();

        let Q1: matrix::Matrix<f64> = vec![
            6.0/7.0, 3.0/7.0, -2.0/7.0,
            3.0/7.0, -2.0/7.0, 6.0/7.0,
            -2.0/7.0, 6.0/7.0, 3.0/7.0
        ].into();
        let Q2: matrix::Matrix<f64> = vec![
            1.0, 0.0, 0.0,
            0.0, -7.0/25.0, 24.0/25.0,
            0.0, 24.0/25.0, 7.0/25.0
        ].into();

        let mut series: _ = HouseholderSeries::prepare_new(A.clone());
        series.push((&Q1).transpose(), Q1.clone());
        series.push((&Q2).transpose(), Q2.clone());

        let test: _ = series.build();

        let exp_Q: matrix::Matrix<f64> = vec![
            0.8571, -0.3943, 0.3314,
            0.4286, 0.9029, -0.0343,
            -0.2857, 0.1714, 0.9429
        ].into();

        for (t,e) in test.Q.into_iter()
            .zip(exp_Q)
        {
            if !t.abs().approx_eq(e.abs(), (0.001, 4)) {
                panic!("{} != {}", t, e)
            }
        }

        let exp_R: matrix::Matrix<f64> = vec![
            14.0, 21.0, -14.0,
            0.0, 175.0, -70.0,
            0.0, 0.0, -35.0
        ].into();

        for (t,e) in test.R.into_iter()
            .zip(exp_R)
        {
            if !t.abs().approx_eq(e.abs(), (0.001, 4)) {
                panic!("{} != {}", t, e)
            }
        }
    }

    #[test]
    fn test_single_round() {
        let matrix: matrix::Matrix<f64> = vec![
            12.0, -51.0, 4.0, 
            6.0, 167.0, -68.0, 
            -4.0, 24.0, -41.0
        ].into();


        let (Q1,_): _ = HouseholderDecomposition::single_round(&matrix, 0); 
        let test: _ = &Q1 * &matrix;  
        let exp_Q1A: matrix::Matrix<f64> = vec![
            14.0, 21.0, -14.0,
            0.0, -49.0, -14.0,
            0.0, 168.0, -77.0
        ].into();
       
        for (t,e) in test.clone().into_iter()
            .zip(exp_Q1A)
        {
            if !t.abs().approx_eq(e.abs(), (0.001, 4)) {
                panic!("{} != {}", t, e)
            }
        }

        let (Q2,_): _ = HouseholderDecomposition::single_round(&test, 1);
        let exp_Q2: matrix::Matrix<f64> = vec![
            1.0, 0.0, 0.0,
            0.0, -7.0/25.0, 24.0/25.0,
            0.0, 24.0/25.0, 7.0/25.0
        ].into();

        for (t,e) in Q2.into_iter()
            .zip(exp_Q2)
        {
            if !t.abs().approx_eq(e.abs(), (0.1, 4)) {
                panic!("{} != {}", t, e)
            }
        }
    }
}
