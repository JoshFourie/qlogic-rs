//! Docs: InProgress, view src.

use crate::matrix;

use matrix::interface;

use interface::{Transpose, Column, Identity};

use crate::vector;

use vector::interface::{Norm, Direct};

use std::ops;

// rayleigh quotient shift: https://ocw.mit.edu/courses/mathematics/18-335j-introduction-to-numerical-methods-fall-2010/readings/MIT18_335JF10_lec16.pdf

// implicit psuedocode: https://people.inf.ethz.ch/arbenz/ewp/Lnotes/chapter4.pdf

impl<T:Copy> interface::QR for matrix::Matrix<T>
where
    T: num::Float
    + ops::Mul<Output=T>
    + From<f32>
{
    type Output = (matrix::Matrix<T>, matrix::Matrix<T>);

    fn qr(mut self) -> Self::Output
    {
        let A: Self = self.clone();
        let col: usize = self.col; 
        let I: &Self = &(&self).identity();

        let mut series_of_Q: Vec<Self> = Vec::with_capacity(col);
        let mut series_of_R: Vec<Self> = Vec::with_capacity(col);

        for k in 0..col-1 {
            let P: Self = HouseholderTransform::new(&self,k).get_householder();
            let mut Q: Self = (&P)*(&self);

            for i in 0..self.row {
                Q[k][i] = T::zero();
                Q[i][k] = T::zero();
            }
            
            series_of_Q.push((&P).transpose());
            series_of_R.push(P);

            Q[k][k] = T::one();
            self = Q;
        }

        let R: Self = series_of_R.into_iter()
            .rev()
            .fold(I.clone(),|acc,Rk| acc * Rk) * A;
        
        let Q: Self = series_of_Q.into_iter()
            .fold(I.clone(), |acc,Qk| acc * Qk);

        (Q,R)
    }
}

struct HouseholderTransform<T> {
    I: matrix::Matrix<T>,
    x: vector::Vector<T>,
    k: usize,
    col: usize 
}

impl<T> HouseholderTransform<T> 
where
    T: num::Float
    + ops::Mul<Output=T>
    + From<f32>
{
    fn new(source_matrix: &matrix::Matrix<T>, k: usize) -> Self {
        Self {
            I: source_matrix.identity(),
            x: source_matrix.get_col(k).into(),
            k,
            col: source_matrix.col
        }
    }

    fn get_householder(self) -> matrix::Matrix<T> 
    {   
        let x = self.x;
        let k = self.k;

        let vk: vector::Vector<T> = {
            // define e1.
            let mut e1: vector::Vector<T> = vec![T::zero(); self.col].into();
            e1[k] = T::one();

            // define alpha as -sign(x[1])||x||
            let alpha: T = -x[k+1].signum() * x.clone().eucl_norm();

            // define vk1 such that sign(x[1])||x||e1 + x.
            let vk1: vector::Vector<T> = e1 * alpha  + x.clone();

            // multiply vk1 by 1/||vk1||
            vk1.clone() * (T::one()/vk1.eucl_norm())
        };

        let householder: matrix::Matrix<T> = 
        {
            // define 2 * vk1 * vk1
            let VV: matrix::Matrix<T> = vk.clone().direct_product(vk);

            // define householder rotation as I - 2vv*
            self.I - (VV * <T as std::convert::From<f32>>::from(2.0))                
        };
        
        householder
    }
}

#[cfg(test)] mod tests 
{
    use crate::matrix;

    use matrix::interface::QR;
    use float_cmp::ApproxEq;

    #[test] fn test_qr_decomposition()
    {
        let matrix: matrix::Matrix<f64> = vec![12.0, -51.0, 4.0, 6.0, 167.0, -68.0, -4.0, 24.0, -41.0].into();

        let exp_Q: matrix::Matrix<f64> = vec![0.8571, -0.3943, 0.3314, 0.4286, 0.9020, -0.0343, -0.2857, 0.1714, 0.9429].into();

        let exp_R: matrix::Matrix<f64> = vec![14.0, 21.0, -14.0, 0.0, 175.0, -70.0, 0.0, 0.0, -35.0].into();

        let (Q,R) = matrix.qr();

        for (test,exp) in Q.into_iter()
            .zip(exp_Q)
            .chain(R.into_iter().zip(exp_R))
        {
            if !test.approx_eq(exp, (0.001, 4)) {
                panic!("{} != {}", test, exp)
            }
        }
    }
}