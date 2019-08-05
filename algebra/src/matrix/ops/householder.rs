use crate::matrix;
use matrix::interface;
use interface::{Column, Identity};

use crate::vector;
use vector::interface::{Norm, Direct};

use num::traits::real;

pub struct Householder<T>(matrix::Matrix<T>);

impl<T> Householder<T> 
where
    T: real::Real + From<f32>
{
    pub fn new(mat: &matrix::Matrix<T>, k: usize) -> Self {
        let helper: _ = Helper::new(mat, k);
        let ae1: _ = AlphaEpsilonOne::new(&helper);
        
        Self::from_ae1(ae1, &helper)
    }

    pub fn from_ae1(ae1: AlphaEpsilonOne<T>, helper: &Helper<'_,T>) -> Self {
        let x: _ = &helper.x;
        let I: _ = helper.mat.identity();

        let V: vector::Vector<T> = {
            let u: vector::Vector<T> = x - &ae1.0;
            u.clone() * (T::one()/u.eucl_norm())
        };

        let householder: matrix::Matrix<T> = {
            let VV: matrix::Matrix<T> = V.clone().direct_product(V);
            I - (VV * <T as std::convert::From<f32>>::from(2.0))                
        };

        Householder(householder)
    }
} 

#[derive(PartialEq, Debug)]
pub struct AlphaEpsilonOne<T>(vector::Vector<T>);

impl<T> AlphaEpsilonOne<T> 
where
    T: real::Real + From<f32>
{
    pub fn new(helper: &Helper<'_,T>) -> Self {
        let col: usize = helper.mat.col;
        let k: usize = helper.k;
        let x: &vector::Vector<T> = &helper.x;

        let mut e1: vector::Vector<T> = vec![T::zero(); col].into();
        e1[k] = T::one();
        // removed signum for testing -x[k+1]
        let alpha: T = x[k+1].signum() * x.clone().eucl_norm();

        AlphaEpsilonOne(e1*alpha)
    }

    pub fn manual(inner: vector::Vector<T>) -> Self {
        AlphaEpsilonOne(inner)
    }
}

pub struct Helper<'a,T> {
    mat: &'a matrix::Matrix<T>,
    x: vector::Vector<T>,
    k: usize,
}

impl<'a,T> Helper<'a,T> 
where
    T: real::Real + From<f32>
{
    pub fn new(mat: &'a matrix::Matrix<T>, k: usize) -> Self {
        Helper {
            mat, 
            x: mat.get_col(k).into(), 
            k
        }
    }
}

impl<T> From<Householder<T>> for matrix::Matrix<T> {
    fn from(householder: Householder<T>) -> Self {
        householder.0
    }
} 

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::ApproxEq;

    fn test_matrix() -> matrix::Matrix<f64> {
        let matrix: matrix::Matrix<f64> = vec![
            12.0, -51.0, 4.0, 
            6.0, 167.0, -68.0, 
            -4.0, 24.0, -41.0
        ].into();
        matrix
    }

    #[test]
    fn test_householder() {
        let matrix: _ = test_matrix();
        let test: matrix::Matrix<f64> = Householder::new(&matrix, 0).into();
        let exp: matrix::Matrix<f64> = vec![
            6.0/7.0, 3.0/7.0, -2.0/7.0,
            3.0/7.0, -2.0/7.0, 6.0/7.0,
            -2.0/7.0, 6.0/7.0, 3.0/7.0
        ].into();
        for (t,e) in test.into_iter()
            .zip(exp)
        {
            if !t.abs().approx_eq(e.abs(), (0.001, 4)) {
                panic!("{} != {}", t, e)
            }
        }
    }

    #[test]
    fn test_householder_zeroing() {
        let matrix: matrix::Matrix<f64> = vec![
            14.0, 21.0, -14.0,
            0.0, -49.0, -14.0,
            0.0, 168.0, -77.0
        ].into();
        let test: _ = Householder::new(&matrix, 1);
        println!("{:?}", test.0);
        panic!("")
    }

    #[test]
    fn test_helper_x_vector() {
        let matrix: _ = test_matrix();
        let test: _ = Helper::new(&matrix, 0).x;
        let exp: vector::Vector<f64> = vec![12.0, 6.0, -4.0].into();
        assert_eq!(test,exp)
    }

    #[test]
    fn test_ae1() {
        let matrix: _ = test_matrix();
        let test: AlphaEpsilonOne<_> = {
            let tmp_helper: _ = Helper::new(&matrix, 0);
            AlphaEpsilonOne::new(&tmp_helper)
        };
        let exp: _ = AlphaEpsilonOne::manual(vec![14.0, 0.0, 0.0].into());
        assert_eq!(test,exp)
    }
}
