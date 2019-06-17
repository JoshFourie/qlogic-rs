//! Docs: InProgress, view src.

use crate::matrix;

use matrix::interface;

use interface::{Transpose, Identity};

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
    + std::fmt::Debug
{
    type Output = (matrix::Matrix<T>, matrix::Matrix<T>);

    fn qr(self) -> Self::Output
    {
        let m = self.row;
        let n = self.col;

        let get_householder = |A: &mut matrix::Matrix<T>, k: usize| -> matrix::Matrix<T> 
        {            
            let x: vector::Vector<T> = {
                let mut buf: Vec<T> = Vec::new();
                for i in 0..m {
                    buf.push(A[i][k])
                }
                vector::Vector::from(buf)
            };

            let vk: vector::Vector<T> = {
                
                // define e1.
                let mut e1: vector::Vector<T> = vec![T::zero(); m].into();
                e1[k] = T::one();

                // define alpha as sign(x[1])||x||
                let alpha: T = -x[k].signum() * x.clone().eucl_norm();

                // define vk1 such that sign(x[1])||x||e1 + x.
                let vk1: vector::Vector<T> = e1 * alpha  + x.clone();

                // multiply vk1 by 1/||vk1||
                vk1.clone() * (T::one()/vk1.eucl_norm())
            };

            let householder: matrix::Matrix<T> = 
            {
                // define I as a matrix m-k+1 x m-k+1
                let I: matrix::Matrix<T> = (&self).identity();

                // define 2 * vk1 * vk1
                let VV: matrix::Matrix<T> = vk.clone().direct_product(vk);

                // define householder rotation as I - 2vv*
                I - (VV * <T as std::convert::From<f32>>::from(2.0))                
            };
            
            householder
        };

        unimplemented!()
    }
}


#[cfg(test)] mod tests 
{
    use crate::matrix;

    use matrix::interface::QR;

    #[test] fn test_qr_decomposition()
    {
        let matrix: matrix::Matrix<f64> = vec![12.0, -51.0, 4.0, 6.0, 167.0, -68.0, -4.0, 24.0, -41.0].into();

        let exp_Q: matrix::Matrix<f64> = vec![0.8571, -0.3943, 0.3314, 0.4286, 0.9020, -0.0343, -0.2857, 0.1714, 0.9429].into();

        let exp_R: matrix::Matrix<f64> = vec![14.0, 21.0, -14.0, 0.0, 175.0, -70.0, 0.0, 0.0, -35.0].into();

        let (Q,R) = matrix.qr();

        println!("\n{:?} \n {:?} \n", Q, exp_Q);

        println!("\n{:?} \n {:?} \n", R, exp_R);

        panic!("")
    }
}