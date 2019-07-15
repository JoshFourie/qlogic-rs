use crate::matrix;
use matrix::interface;
use interface::{Column, Identity};

use crate::vector;
use vector::interface::{Norm, Direct};

use num::traits::real;

pub struct HouseholderReflection<T> {
    I: matrix::Matrix<T>,
    x: vector::Vector<T>,
    k: usize,
    col: usize
}

impl<T> HouseholderReflection<T> 
where
    T: real::Real
    + From<f32>
{
    pub fn new(source_matrix: &matrix::Matrix<T>, k: usize) -> Self 
    {
        HouseholderReflection {
            I: source_matrix.identity(),
            x: source_matrix.get_col(k).into(),
            k,
            col: source_matrix.col
        }
    }

    pub fn reflector(self) -> matrix::Matrix<T> 
    {   
        let alpha_e1: vector::Vector<T> = {
            let x = &self.x;
            let k = self.k;

            let mut e1: vector::Vector<T> = vec![T::zero(); self.col].into();
            e1[k] = T::one();
            let alpha: T = -x[k+1].signum() * x.clone().eucl_norm();

            e1*alpha
        };
        
        self.reflector_with_ae1(alpha_e1)        
    }

    pub fn reflector_with_ae1(self, alpha_e1: vector::Vector<T>) -> matrix::Matrix<T> 
    {
        let V: vector::Vector<T> = {
            let u: vector::Vector<T> = self.x - alpha_e1;
            u.clone() * (T::one()/u.eucl_norm())
        };

        let householder: matrix::Matrix<T> = {
            let VV: matrix::Matrix<T> = V.clone().direct_product(V);
            self.I - (VV * <T as std::convert::From<f32>>::from(2.0))                
        };
        
        householder
    }
} 
